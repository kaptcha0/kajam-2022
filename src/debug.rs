use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::{health::Health, player::Player};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Health>()
            .register_inspectable::<Player>();
    }
}
