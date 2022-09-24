use bevy::prelude::{Component, Bundle};
use bevy_inspector_egui::Inspectable;

use crate::{elements::Element, chunk::{ChunkId, ChunkSize}};

#[derive(Default, Component, Inspectable, Clone)]
pub struct Cell {
    pub element: Element,
}

#[derive(Component, Default)]
pub struct CellPos {
    x: u32,
    y: u32,
}

impl CellPos {
    pub fn index(&self, size: ChunkSize) -> usize {
        (self.y * size.height + self.x) as usize
    }
}

#[derive(Bundle, Default)]
pub struct CellBundle {
    pub element: Element,
    pub cell_pos: CellPos,
    pub chunk_id: ChunkId,
}
