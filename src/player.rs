use crate::{block_type::BLOCK_SIZE, utils::Layers};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use heron::prelude::*;

pub const PLAYER_SIZE: f32 = BLOCK_SIZE / 2.0;
const SPAWN_POINT: [f32; 3] = [0.0, 0.75, 999.0];

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(camera_follow)
            .add_system(reset_player);
    }
}

#[derive(Component)]
pub struct Limits {
    max_velocity: Vec2,
}

#[derive(Component, Inspectable)]
pub struct Player {
    pub speed: f32,
    pub jump_height: f32,
    pub jumped: bool,
    pub peppers: u32,
}

fn spawn_player(mut commands: Commands) {
    let geometry = load_player_sprite();

    commands
        .spawn_bundle(geometry)
        .insert(Name::new("Player"))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::X * 0.0))
        .insert(Acceleration::default())
        .insert(CollisionShape::Sphere {
            radius: PLAYER_SIZE,
        })
        .insert(PhysicMaterial {
            restitution: 0.35,
            friction: 0.0, //PLAYER_SIZE / 10.0,
            density: PLAYER_SIZE,
        })
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::none()
                .with_group(Layers::Player)
                .with_masks(&[Layers::Enemy, Layers::Level, Layers::Pepper]),
        )
        .insert(Player {
            speed: 100.0,
            jump_height: 1500.0,
            jumped: false,
            peppers: 0,
        })
        .insert(Limits {
            max_velocity: Vec2::new(2000.0, 3000.0),
        });
}

fn load_player_sprite() -> impl Bundle {
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

    return SpriteBundle {
        sprite,
        transform: Transform::default().with_translation(SPAWN_POINT.into()),
        ..Default::default()
    };
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &Limits, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let (player, limits, mut velocity) = player_query.single_mut();

    let max_y = limits.max_velocity.y * PLAYER_SIZE * delta;
    let max_x = limits.max_velocity.x * PLAYER_SIZE * delta;

    velocity.linear.y += PLAYER_SIZE * delta;

    if !player.jumped && velocity.linear.y.abs() <= max_y {
        if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Space) {
            velocity.linear.y += player.jump_height * PLAYER_SIZE * delta;
        }
    }

    if (velocity.linear.x >= -max_x)
        && (keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left))
    {
        velocity.linear.x += -player.speed * PLAYER_SIZE * delta;
    }

    if (velocity.linear.x <= max_x)
        && (keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right))
    {
        velocity.linear.x += player.speed * PLAYER_SIZE * delta;
    }
}

fn reset_player(mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>) {
    let (mut transform, mut velocity) = player_query.single_mut();

    if transform.translation.y < -1.5 {
        transform.translation = SPAWN_POINT.into();
        let new_vel = Velocity::from_linear(Vec3::X * 0.0);
        velocity.linear = new_vel.linear;
        velocity.angular = new_vel.angular;
    }
}
