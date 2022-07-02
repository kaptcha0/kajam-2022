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
            .add_system(can_jump)
            .add_system(reset_player);
    }
}

#[derive(Component)]
pub struct Limits {
    max_velocity: Vec2,
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    jump_height: f32,
    jumped: bool,
}

fn spawn_player(mut commands: Commands) {
    let geometry = load_player_sprite();

    commands
        .spawn_bundle(geometry)
        .insert(Name::new("Player"))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::X * 0.0))
        .insert(Acceleration::default())
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::splat(PLAYER_SIZE / 2.0),
            border_radius: Some(0.015),
        })
        .insert(PhysicMaterial {
            restitution: 0.35,
            friction: PLAYER_SIZE / 3.0,
            density: PLAYER_SIZE,
        })
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::none()
                .with_group(Layers::Player)
                .with_masks(&[Layers::Enemy, Layers::Level]),
        )
        .insert(Player {
            speed: 100.0,
            jump_height: 1500.0,
            jumped: false,
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

fn can_jump(mut player_query: Query<&mut Player>, mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        let (is_player, is_level) = collided_level(event.collision_layers());
        let entities = event.rigid_body_entities();

        let player = if is_player {
            Some(entities.0)
        } else if is_level {
            Some(entities.1)
        } else {
            None
        };

        let contact = match event {
            CollisionEvent::Started(_, _) => true,
            CollisionEvent::Stopped(_, _) => false,
        };

        match player {
            Some(_) => {
                let mut player = player_query.single_mut();
                player.jumped = !contact;
            }
            None => {}
        };
    }
}

fn collided_level(layers: (CollisionLayers, CollisionLayers)) -> (bool, bool) {
    let (l1, l2) = layers;

    let is_player = l1.contains_group(Layers::Player) && l2.contains_group(Layers::Level);
    let is_level = l1.contains_group(Layers::Level) && l2.contains_group(Layers::Player);

    (is_player, is_level)
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
