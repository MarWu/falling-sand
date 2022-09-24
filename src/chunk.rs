use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureFormat},
        texture::ImageSampler,
    },
};

use crate::{cell::{self, CellPos, CellBundle}, elements::Element, CHUNK_SCALE, CHUNK_SIZE, helper::create_chunk_image};
use bevy_inspector_egui::Inspectable;
use cell::Cell;

#[derive(Bundle)]
pub struct ChunkBundle {
    chunk_pos: ChunkPos,
    chunk_size: ChunkSize,
    chunk_scale: ChunkScale,
    texture_handle: ChunkTexture,
    cell_storage: CellStorage,
}

#[derive(Component, Inspectable)]
pub struct Chunk {
    pub width: usize,
    pub height: usize,
    pub pos: Vec2,
    scale: Vec2,
    pub cells: Vec<Cell>,
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct ChunkPos {
    x: i64,
    y: i64,
}

#[derive(Component)]
pub struct ChunkSize {
    pub width: u32,
    pub height: u32,
}

impl ChunkSize {
    fn square(len: u32) -> Self {
        Self { width: len, height: len }
    }
}

impl From<u32> for ChunkSize {
    fn from(len: u32) -> Self {
        ChunkSize { width: len, height: len }
    }
}

#[derive(Component)]
pub struct ChunkScale(pub u64);

#[derive(Component)]
pub struct ChunkTexture(pub Handle<Image>);

#[derive(Component)]
pub struct CellStorage {
    storage: Vec<Option<Entity>>,
    size: ChunkSize,
}

impl CellStorage {
    fn empty(size: ChunkSize) -> Self {
        Self {
            storage: vec![None; (size.width * size.height) as usize],
            size,
        }
    }

    fn set(&self, cell_pos: &CellPos, cell_entity: Option<Entity>) {
        self.storage[cell_pos.index(self.size)] = cell_entity;
    }
}

#[derive(Component, Clone, Copy, Debug, Hash)]
pub struct ChunkId(pub Entity);

impl Default for ChunkId {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

impl Chunk {
    pub fn new(width: usize, height: usize, pos: Vec2, scale: Vec2, image: Handle<Image>) -> Self {
        Chunk {
            width: CHUNK_SIZE,
            height: CHUNK_SIZE,
            pos,
            scale,
            cells: vec![
                Cell {
                    element: Element::Air
                };
                width * height
            ],
            image,
        }
    }

    pub fn set_floor(&mut self) {
        for i in 0..self.width {
            let index = self.index(i, self.height - 1);
            self.cells[index].element = Element::Stone;
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.height + x
    }

    pub fn set(&mut self, coords: Vec2) {
        let index = self.index(coords.x as usize, coords.y as usize);
        self.cells[index].element = Element::Sand;
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.cells.swap(a, b);
    }

    pub fn is_empty(&self, index: usize) -> bool {
        self.cells[index].element == Element::Air
    }

    fn origin(&self) -> Vec2 {
        self.pos
            - Vec2::new(
                self.width as f32 * self.scale.x / 2.,
                self.height as f32 * self.scale.y / 2.,
            )
    }

    pub fn coord_from_world_pos(&self, world_pos: Vec2) -> Vec2 {
        let pixel_pos = Vec2::new(
            world_pos.x - self.origin().x,
            -(world_pos.y + self.origin().y),
        );
        (pixel_pos / self.scale).floor()
    }
}

pub fn spawn_chunk(mut commands: Commands, mut images: ResMut<Assets<Image>>, pos: Vec2) -> Entity {
    let chunk_entity = commands.spawn().id();
    let mut cell_storage = CellStorage::empty(CHUNK_SIZE.into());
    let chunk_size = ChunkSize::square(CHUNK_SIZE);
    
    for x in 0..chunk_size.width {
        for y in 0 ..chunk_size.height {
            let cell_pos = CellPos { x, y };
            let cell_entity = commands.spawn()
                .insert_bundle(CellBundle {
                    cell_pos,
                    chunk_id: ChunkId(chunk_entity),
                    ..Default::default()
                })
                .id();
            commands.entity(chunk_entity).add_child(cell_entity);
            cell_storage.set(&cell_pos, Some(cell_entity));
        }
    }

    // let scale = Vec2::new(CHUNK_SCALE as f32, CHUNK_SCALE as f32);
    let image_handle = images.add(create_chunk_image(chunk_size.width as u32, chunk_size.height as u32));


    let mut chunk = Chunk::new(CHUNK_SIZE, CHUNK_SIZE, pos, scale, image_handle.clone());
    chunk.set_floor();
    let entity = commands.spawn().insert(chunk).id();

    commands.spawn().insert_bundle(SpriteBundle {
        texture: image_handle,
        transform: Transform {
            translation: Vec3::new(pos.x, pos.y, 0.),
            scale: Vec3::new(scale.x, scale.y, 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    entity
}
