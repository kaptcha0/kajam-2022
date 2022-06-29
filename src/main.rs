mod block_type;
mod debug;
mod player;
mod terrain;
mod utilities;

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use heron::prelude::*;

use debug::DebugPlugin;
use player::PlayerPlugin;
use terrain::TerrainPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    let height = 720.0;
    let mut app = App::new();

    // Boilerplate things
    app.insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "Test app".to_string(),
            width: height * RESOLUTION,
            height,
            present_mode: PresentMode::Fifo,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -1.0, 0.0)))
        .add_plugin(DebugPlugin);

    // Camera
    app.add_startup_system(spawn_camera);

    app.add_plugin(TerrainPlugin).add_plugin(PlayerPlugin);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
