use bevy::prelude::*;

use bevy_inspector_egui::Inspectable;
use cell::Cell;
use crate::{elements::Element, cell, CHUNK_SIZE};

#[derive(Component, Inspectable)]
pub struct Chunk {
    width: usize,
    height: usize,
    pos: Vec2,
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
            Cell {element: Element::Air};
            width * height
        ],
        image,
        }
    }

    pub fn set_border(&mut self) {
        for i in 0..self.height {
            let index = self.index(0, i);
            self.cells[index].element = Element::Sand;
            let index = self.index(self.width - 1, i);
            self.cells[index].element = Element::Sand;
        }
    }

    pub fn set_test(&mut self) {
        self.cells[0].element = Element::Air;
        self.cells[1].element = Element::Sand;
        self.cells[2].element = Element::Stone;
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.height + x
    }

    fn origin(&self) -> Vec2 {
        let origin = self.pos - Vec2::new(self.width as f32 * self.scale.x / 2., self.height as f32 * self.scale.y / 2.);
        println!("self.pos: {}", self.pos);
        println!("origin: {}", origin);
        origin
    }

    pub fn coord_from_world_pos(&self, world_pos: Vec2) -> Vec2 {
        Vec2::new(world_pos.x - self.origin().x, - world_pos.y + self.origin().y)
    }
}
