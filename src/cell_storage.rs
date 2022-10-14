use bevy::prelude::*;

use crate::chunk::ChunkSize;

#[derive(Component, Debug)]
pub struct CellStorage {
    cells: Vec<Option<Entity>>,
    pub size: ChunkSize,
}

impl CellStorage {
    pub fn empty(size: ChunkSize) -> Self {
        Self {
            cells: vec![None; size.count()],
            size,
        }
    }

    pub fn set(&self, cell_pos: &crate::cell::CellPos, cell_entity: Entity) {
        self.cells[cell_pos.to_index(self.size)].replace(cell_entity);
    }
}
