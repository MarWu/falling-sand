use bevy::{prelude::*, utils::HashMap};

use crate::CHUNK_SIZE;

struct World {
    chunks: HashMap<(usize, usize), Entity>,
    chunk_size: usize,
    scale: usize,
}

impl World {
    fn empty(scale: usize) -> Self {
        World {
            chunks: HashMap::new(),
            chunk_size: CHUNK_SIZE,
            scale,
        }
    }

    fn add_chunk(&mut self, coord: Vec2, entity: Entity) {
        self.chunks.insert((coord.x as usize, coord.y as usize), entity);
    }
}
