use bevy::prelude::*;

use crate::{chunk::Chunk, BYTES_PER_PIXEL};

pub fn texture_updater(
    chunks: Query<&Chunk>,
    mut images: ResMut<Assets<Image>>,
) {
    let chunk = chunks.single();
    let image = images.get_mut(&chunk.image).unwrap();
    for (i, cell) in (chunk.cells).iter().enumerate() {
        let color = cell.element.get_color();
        let index = i * BYTES_PER_PIXEL;
        image.data[index] = color[0];
        image.data[index + 1] = color[1];
        image.data[index + 2] = color[2];
        image.data[index + 3] = color[3];
    }
}
