use kajam_lib::collisions::CollisionsPlugin;
#[cfg(debug_assertions)]
use kajam_lib::debug::DebugPlugin;

use kajam_lib::pepper::PepperPlugin;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_prototype_lyon::prelude::ShapePlugin;
use heron::prelude::*;

use kajam_lib::assets::AssetsPlugin;
use kajam_lib::fireball::FireballPlugin;
use kajam_lib::player::PlayerPlugin;
use kajam_lib::terrain::TerrainPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    init()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
fn init() {
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
        // .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -5.0, 0.0)));

    if cfg!(debug_assertions) {
        app.add_plugin(DebugPlugin);
    }

    // Camera
    app.add_startup_system(spawn_camera);

    app.add_plugin(AssetsPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(FireballPlugin)
        .add_plugin(PepperPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CollisionsPlugin);

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
