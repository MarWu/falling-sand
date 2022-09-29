use bevy::prelude::*;

use crate::{chunk::Chunk, BYTES_PER_PIXEL};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(texture_updater);
    }
}

pub fn texture_updater(chunks: Query<&Chunk>, mut images: ResMut<Assets<Image>>) {
    for chunk in chunks.iter() {
        let image = images.get_mut(&chunk.image).unwrap();
        for (i, cell) in (chunk.cells).iter().enumerate() {
            match cell {
                Some(cell) => {
                    let color = cell.element.get_color();
                    let index = i * BYTES_PER_PIXEL;
                    image.data[index] = color[0];
                    image.data[index + 1] = color[1];
                    image.data[index + 2] = color[2];
                    image.data[index + 3] = color[3];
                }
                None => (),
            }
        }
    }
}
