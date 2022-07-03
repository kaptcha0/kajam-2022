use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};
use rand::Rng;

use crate::{
    assets::GameAssets, block_type::BLOCK_SIZE, player::PLAYER_SIZE, terrain::MAP_LEN,
    utils::Layers,
};

pub struct PepperPlugin;

impl Plugin for PepperPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_peppers)
            .add_system(hover_effect);
    }
}

#[derive(Component)]
pub struct Pepper {
    id: f32,
}

fn spawn_peppers(mut commands: Commands, assets: Res<GameAssets>) {
    let mut rng = rand::thread_rng();
    let pepper_count = MAP_LEN / 10_u32;
    let range = (
        Vec2::splat(BLOCK_SIZE * 5.0),
        Vec2::splat((MAP_LEN as f32 * BLOCK_SIZE) - (BLOCK_SIZE * 5.0)),
    );

    for i in 0..pepper_count {
        let noise = rng.gen::<f32>() * 10.0;
        let x = range.0.lerp(range.1, i as f32 / pepper_count as f32).x + (noise * BLOCK_SIZE);

        let loc = Vec3::new(x, 0.75, 800.0);

        let bundle = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(PLAYER_SIZE * 0.8)),
                ..Default::default()
            },
            texture: assets.pepper.clone_weak(),
            transform: Transform {
                translation: loc,
                ..Default::default()
            },
            ..Default::default()
        };

        commands
            .spawn_bundle(bundle)
            .insert(Name::new("pepper"))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::splat(PLAYER_SIZE * 0.8),
                border_radius: None,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layers::Pepper)
                    .with_masks(&[Layers::Player, Layers::Level]),
            )
            .insert(RotationConstraints::lock())
            .insert(Pepper {
                id: rng.gen::<f32>() * 1.5,
            });
    }
}

fn hover_effect(mut query: Query<(&mut Transform, &Pepper)>, time: Res<Time>) {
    let elapsed = time.seconds_since_startup() as f32;
    let delta = time.delta_seconds();

    for (mut transform, pepper) in query.iter_mut() {
        transform.translation.y +=
            3.25 * ((0.2 * (elapsed + pepper.id)).cos().to_radians() * delta);
    }
}
