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
    pub pos: IVec2,
    scale: UVec2,
    pub cells: Vec<Option<Cell>>,
    pub image: Handle<Image>,
}

impl Chunk {
    pub fn new(width: usize, height: usize, pos: IVec2, scale: UVec2, image: Handle<Image>) -> Self {
        Chunk {
            width: CHUNK_SIZE,
            height: CHUNK_SIZE,
            pos,
            scale,
            cells: vec![
                None;
                width * height
            ],
            image,
        }
    }

    // pub fn set_floor(&mut self) {
    //     for i in 0..self.width {
    //         let index = self.index(i, self.height - 1);
    //         self.cells[index].element = Element::Stone;
    //     }
    // }

    pub fn index(&self, x: usize, y: usize) -> usize {
        self.width * (self.height - y - 1) + x
        // y * self.height + x
    }

    pub fn set(&mut self, coords: IVec2) {
        let index = self.index(coords.x as usize, coords.y as usize);
        self.cells[index] = Some(Cell::new(Element::Sand));
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.cells.swap(a, b);
    }

    pub fn is_empty(&self, index: usize) -> bool {
        self.cells[index].is_none()
    }

    pub fn get_neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = vec![];
        neighbors.push(index + self.height - 1);
        neighbors.push(index + self.height + 1);
        neighbors
    }
}

pub fn spawn_chunk(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    pos: IVec2,
) -> Entity {
    let width = CHUNK_SIZE as u32;
    let height = CHUNK_SIZE as u32;
    let scale = UVec2::new(CHUNK_SCALE as u32, CHUNK_SCALE as u32);
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
    let entity = commands.spawn().insert(chunk).id();

    commands.spawn().insert_bundle(SpriteBundle {
        texture: image_handle,
        transform: Transform {
            translation: Vec3::new((pos.x * width as i32 * scale.x as i32) as f32, (pos.y * height as i32 * scale.y as i32) as f32, 0.),
            scale: Vec3::new(scale.x as f32, scale.y as f32, 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    entity
}
