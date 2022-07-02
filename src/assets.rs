use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets);
    }
}

pub struct GameAssets {
    pub fireball: Handle<Image>,
    pub pepper: Handle<Image>,
}

fn load_assets(mut commands: Commands, server: Res<AssetServer>) {
    let fireball = server.load("fireball.png");
    let pepper = server.load("pepper.png");
    info!("Loaded assets");

    commands.insert_resource(GameAssets { fireball, pepper })
}
