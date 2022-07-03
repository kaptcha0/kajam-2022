#[cfg(debug_assertions)]
use kajam_lib::debug::DebugPlugin;

use kajam_lib::health::HealthPlugin;
use kajam_lib::hud::HudPlugin;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::ShapePlugin;
use heron::prelude::*;

use kajam_lib::assets::AssetsPlugin;
use kajam_lib::camera::{CameraBundle, CLEAR, RESOLUTION};
use kajam_lib::collisions::CollisionsPlugin;
use kajam_lib::enemy::EnemyPlugin;
use kajam_lib::fireball::FireballPlugin;
use kajam_lib::pepper::PepperPlugin;
use kajam_lib::player::PlayerPlugin;
use kajam_lib::terrain::TerrainPlugin;

fn main() {
    init()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn init() {
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
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -5.0, 0.0)));

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    // Camera
    app.add_plugin(AssetsPlugin)
        .add_plugin(CameraBundle)
        .add_plugin(HudPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(FireballPlugin)
        .add_plugin(PepperPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollisionsPlugin)
        .add_plugin(HealthPlugin);

    app.run();
}
