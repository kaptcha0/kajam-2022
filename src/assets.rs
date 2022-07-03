use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
    }
}

pub struct Fonts {
    pub regular: Handle<Font>,
    pub medium: Handle<Font>,
}

pub struct GameAssets {
    pub fireball: Handle<Image>,
    pub pepper: Handle<Image>,
    pub fonts: Fonts,
}

fn load_assets(mut commands: Commands, server: Res<AssetServer>) {
    let fireball = server.load("fireball.png");
    let pepper = server.load("pepper.png");
    let fonts = Fonts {
        regular: server.load("fonts/Roboto-Regular.ttf"),
        medium: server.load("fonts/Roboto-Medium.ttf"),
    };
    info!("Loaded assets");

    commands.insert_resource(GameAssets {
        fireball,
        pepper,
        fonts,
    })
}
