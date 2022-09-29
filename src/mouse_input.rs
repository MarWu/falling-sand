use bevy::{prelude::*, render::camera::CameraProjection};
use bevy_inspector_egui::Inspectable;

use crate::chunk::Chunk;
use crate::world::World;

pub struct MouseStatePlugin;

impl Plugin for MouseStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseInputState>()
            .add_system(mouse_input_handler)
            // .add_system(mouse_chunk_pos) 
            .add_system(draw_cell)
        ;
    }
}

#[derive(Default, Inspectable, Reflect)]
pub struct MouseInputState {
    pub left_button_down: bool,
    pub world_pos: Vec2,
    pub cell_coord: Vec2,
}

pub fn mouse_input_handler(
    mut state: ResMut<MouseInputState>,
    mouse_button: Res<Input<MouseButton>>,
    cameras: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
    wnds: ResMut<Windows>,
) {
    state.left_button_down = mouse_button.pressed(MouseButton::Left);

    let (camera, camera_transform) = cameras.single();
    let wnd = wnds.get_primary().unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2. - Vec2::ONE;
        let ndc_to_world = camera.compute_matrix() * camera_transform.get_projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.));
        let world_pos: Vec2 = world_pos.truncate();
        state.world_pos = world_pos;
    }
}

fn draw_cell(
    state: ResMut<MouseInputState>,
    mut world: ResMut<World>,
    chunks: Query<&mut Chunk>,
) {
    if state.left_button_down {
        world.set_cell(state.world_pos, chunks);
    }
}

// fn mouse_chunk_pos(
//     mut state: ResMut<MouseInputState>,
//     chunk: Query<&Chunk>,
// ) {
//     let chunk = chunk.single();
//     state.cell_coord = chunk.coord_from_world_pos(state.world_pos);
// }
