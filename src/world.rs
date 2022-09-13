use bevy::{prelude::*, utils::HashMap};

use crate::{CHUNK_SIZE, chunk::{self, Chunk}, CHUNK_SCALE, helper::key_from_coords};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World>()
            .add_startup_system(spawn_chunk)
        ;
    }
}

fn spawn_chunk(commands: Commands, images: ResMut<Assets<Image>>, mut world: ResMut<World>) {
    let pos = Vec2::new(0., 0.);
    let entity = chunk::spawn_chunk(commands, images, pos);
    world.add_chunk(pos, entity)
}

pub struct World {
    chunks: HashMap<u64, Entity>,
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
            chunks: HashMap::new(),
            chunk_size: CHUNK_SIZE,
            scale: CHUNK_SCALE,
        }
    }

    fn add_chunk(&mut self, coord: Vec2, entity: Entity) {
        self.chunks.insert(key_from_coords(coord.x as i32, coord.y as i32), entity);
    }

    pub fn set_cell(&mut self, mut world_pos: Vec2, mut chunks: Query<&mut Chunk>) {
        world_pos.y *= -1.;
        let mut pixel_pos = (world_pos / self.scale as f32).floor();
        pixel_pos += (self.chunk_size / 2) as f32;
        let chunk_coord = (pixel_pos / self.chunk_size as f32).floor();
        let key = key_from_coords(chunk_coord.x as i32, chunk_coord.y as i32);

        if self.chunks.contains_key(&key) {
            for mut chunk in chunks.iter_mut() {
                if chunk_coord == chunk.pos {
                    chunk.set(pixel_pos);
                }
            }
        }
    }
}
