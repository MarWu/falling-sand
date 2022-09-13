use bevy::prelude::*;

use crate::{chunk::Chunk, elements::Element};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cell_gravity)
        ;
    }
}

fn cell_gravity(
    mut chunks: Query<&mut Chunk>,
) {
    for mut chunk in chunks.iter_mut() {
        for index in (0..((chunk.width - 1) * (chunk.height - 1))).rev() {
            if chunk.cells[index].element == Element::Sand {
                let below = index + chunk.width;
                if chunk.is_empty(below) {
                    chunk.swap(index, below);
                }
            }
        }
    }
}
