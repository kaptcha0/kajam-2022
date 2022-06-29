use crate::block_type::BLOCK_SIZE;
use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
};
use bevy_inspector_egui::Inspectable;
use heron::prelude::*;

pub const PLAYER_SIZE: f32 = BLOCK_SIZE / 2.0;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    jump_height: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(load_player_sprite)
            .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let (player, mut velocity): (&Player, Mut<Velocity>) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Space) {
        velocity.linear.y += player.jump_height * PLAYER_SIZE * delta;
    }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        velocity.linear.x += -player.speed * PLAYER_SIZE * delta;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        velocity.linear.x += player.speed * PLAYER_SIZE * delta;
    }
}

fn load_player_sprite(mut commands: Commands) {
    let sprite = Sprite {
        color: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        },
        custom_size: Some(Vec2::splat(PLAYER_SIZE)),
        ..Default::default()
    };

    let bundle = SpriteBundle {
        sprite,
        transform: Transform::default().with_translation(Vec3::new(0.0, 0.75, 0.0)),
        ..Default::default()
    };

    commands
        .spawn_bundle(bundle)
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_angular(AxisAngle::new(Vec3::X, PLAYER_SIZE)))
        .insert(Acceleration::default())
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::splat(PLAYER_SIZE / 2.0),
            border_radius: None,
        })
        .insert(PhysicMaterial {
            friction: 1.0,
            density: PLAYER_SIZE,
            ..Default::default()
        })
        .insert(CollisionLayers::default())
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 5.0,
            jump_height: 10.0,
        });
}
