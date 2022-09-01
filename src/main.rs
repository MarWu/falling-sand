use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureFormat}, texture::ImageSampler}, sprite::MaterialMesh2dBundle};
use chunk::Chunk;
use debug::DebugPlugin;
use mouse_input::MouseStatePlugin;
use render::texture_updater;

mod elements;
mod cell;
mod chunk;
mod render;
mod mouse_input;
mod debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MouseStatePlugin)
        .add_startup_system(setup)
        .add_system(texture_updater)
        .add_plugin(DebugPlugin)
        // .add_startup_system(spawn_coord_markers)
        .run();
}

pub const CHUNK_SIZE: usize = 64;
pub const BYTES_PER_PIXEL: usize = 4;

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let width = CHUNK_SIZE as u32;
    let height = CHUNK_SIZE as u32;
    let pos = Vec2::new(0., 0.);
    let scale = Vec2::new(3., 3.);
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 255, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    );
    image.sampler_descriptor = ImageSampler::nearest();

    let image_handle = images.add(image);

    let mut chunk = Chunk::new(CHUNK_SIZE, CHUNK_SIZE, pos, scale, image_handle.clone());
    chunk.set_border();
    chunk.set_test();
    commands.spawn().insert(chunk);
    // commands.spawn().insert(Chunk::new(CHUNK_SIZE, CHUNK_SIZE, image_handle.clone()));
    commands.spawn().insert_bundle(SpriteBundle {
        texture: image_handle,
        transform: Transform { 
            translation: Vec3::new(pos.x, pos.y, 0.), 
            scale: Vec3::new(scale.x, scale.y, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_coord_markers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform { 
            translation: Vec3 { x: 100., y: 100., z: 10. },
            scale: Vec3 { x: 10., y: 10., z: 1. },
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::MAROON)),
        ..default()
    });
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform { 
            translation: Vec3 { x: 150., y: 150., z: 10. },
            scale: Vec3 { x: 10., y: 10., z: 1. },
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::MAROON)),
        ..default()
    });
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform { 
            translation: Vec3 { x: 0., y: 0., z: 10. },
            scale: Vec3 { x: 10., y: 10., z: 1. },
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::MAROON)),
        ..default()
    });
}
