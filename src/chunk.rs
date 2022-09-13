use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureFormat},
        texture::ImageSampler,
    },
};

use crate::{cell, elements::Element, CHUNK_SCALE, CHUNK_SIZE};
use bevy_inspector_egui::Inspectable;
use cell::Cell;

#[derive(Component, Inspectable)]
pub struct Chunk {
    pub width: usize,
    pub height: usize,
    pub pos: Vec2,
    scale: Vec2,
    pub cells: Vec<Cell>,
    pub image: Handle<Image>,
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
    let width = CHUNK_SIZE as u32;
    let height = CHUNK_SIZE as u32;
    let scale = Vec2::new(CHUNK_SCALE as f32, CHUNK_SCALE as f32);
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 255, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    );
    image.sampler_descriptor = ImageSampler::nearest();

    let image_handle = images.add(image);

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
