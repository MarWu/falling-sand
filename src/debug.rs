use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin, InspectorPlugin};

use crate::{chunk::Chunk, mouse_input::MouseInputState};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Chunk>()
            .add_plugin(InspectorPlugin::<MouseInputState>::new())
            .add_plugin(InspectorPlugin::<World>::new())
        ;
    }
}
