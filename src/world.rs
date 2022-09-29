use bevy::{
    prelude::*,
    utils::HashSet,
};

use crate::{
    chunk::{self, Chunk},
    CHUNK_SCALE, CHUNK_SIZE, helper::world_pos_to_coords,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World>()
            .add_system(spawn_chunks_around_camera);
    }
}

// fn spawn_chunk(commands: Commands, images: ResMut<Assets<Image>>, mut world: ResMut<World>) {
//     let entity = chunk::spawn_chunk(commands, images, pos);
// }

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size = IVec2::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32);
    let tile_size = IVec2::new(CHUNK_SCALE as i32, CHUNK_SCALE as i32);
    camera_pos / (chunk_size * tile_size)
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    camera_query: Query<&Transform, With<Camera>>,
    mut world: ResMut<World>,
) {
    let transform = camera_query.single();
    let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.truncate());
    for y in (camera_chunk_pos.y - 0)..(camera_chunk_pos.y + 2) {
        for x in (camera_chunk_pos.x - 1)..(camera_chunk_pos.x + 2) {
            if !world.chunks.contains(&IVec2::new(x, y)) {
                world.chunks.insert(IVec2::new(x, y));
                chunk::spawn_chunk(&mut commands, &mut images, IVec2::new(x, y));
            }
        }
    }
}

pub struct World {
    chunks: HashSet<IVec2>,
    chunk_size: usize,
    scale: usize,
}

impl Default for World {
    fn default() -> Self {
        World::empty()
    }
}

impl World {
    fn empty() -> Self {
        World {
            chunks: HashSet::new(),
            chunk_size: CHUNK_SIZE,
            scale: CHUNK_SCALE,
        }
    }

    pub fn set_cell(&mut self, mut world_pos: Vec2, mut chunks: Query<&mut Chunk>) {
        let (chunk_coord, chunk_pixel) = world_pos_to_coords(world_pos, self.chunk_size, self.scale);

        if self.chunks.contains(&chunk_coord) {
            for mut chunk in chunks.iter_mut() {
                if chunk_coord == chunk.pos {
                    chunk.set(chunk_pixel);
                }
            }
        }
    }
}
