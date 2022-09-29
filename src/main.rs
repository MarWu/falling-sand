use bevy::prelude::*;
use debug::DebugPlugin;
use mouse_input::MouseStatePlugin;
use physics::PhysicsPlugin;
use render::RenderPlugin;
use world::WorldPlugin;

mod cell;
mod chunk;
mod debug;
mod elements;
mod mouse_input;
mod render;
mod physics;
mod world;
mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MouseStatePlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
        .add_plugin(DebugPlugin)
        .run();
}

pub const CHUNK_SIZE: usize = 50;
pub const CHUNK_SCALE: usize = 5;
pub const BYTES_PER_PIXEL: usize = 4;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
