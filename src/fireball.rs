use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    player::{Player, PLAYER_SIZE},
};

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnFireballEvent>()
            .insert_resource(Fireballs { entity: None })
            .add_system(create_fireball)
            .add_system(spawn_fireballs)
            .add_system(move_fireballs);
    }
}

pub struct SpawnFireballEvent {
    spawn_point: Vec3,
    target: Vec3,
}

#[derive(Component)]
pub struct Fireball {
    speed: f32,
    target: Vec3,
}

struct Fireballs {
    entity: Option<Entity>,
}

fn create_fireball(
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    player_query: Query<&Transform, With<Player>>,
    mut event_writer: EventWriter<SpawnFireballEvent>,
) {
    if
    /* mouse.pressed(MouseButton::Left) || */
    mouse.just_pressed(MouseButton::Left) {
        std::thread::sleep(std::time::Duration::from_millis(250));
        let window = windows.get_primary().unwrap();
        let player = player_query.single();

        if let Some(position) = window.cursor_position() {
            let ev = SpawnFireballEvent {
                spawn_point: player.translation,
                target: position.extend(900.0),
            };

            event_writer.send(ev);
        }
    }
}

fn spawn_fireballs(
    mut commands: Commands,
    mut fireballs_entity: ResMut<Fireballs>,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnFireballEvent>,
) {
    let mut fireballs = Vec::<Entity>::new();

    for ev in events.iter() {
        let mut bundle = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(PLAYER_SIZE * 2.0)),
                ..Default::default()
            },
            texture: assets.fireball.clone_weak(),
            transform: Transform {
                translation: ev.spawn_point,
                ..Default::default()
            },
            ..Default::default()
        };

        bundle.transform.look_at(ev.target, Vec3::Y);

        let e = commands
            .spawn_bundle(bundle)
            .insert(Fireball {
                speed: 100.0,
                target: ev.target,
            })
            .id();

        fireballs.push(e);
    }

    if fireballs.len() > 0 {
        match fireballs_entity.entity {
            Some(entity) => {
                commands
                    .get_or_spawn(entity)
                    .insert(Name::new("Fireballs"))
                    .push_children(fireballs.as_slice());
            }
            None => {
                fireballs_entity.entity = Some(
                    commands
                        .spawn()
                        .insert(Name::new("Fireballs"))
                        .push_children(fireballs.as_slice())
                        .id(),
                );
            }
        }
    }
}

fn move_fireballs(mut query: Query<(&mut Transform, &Fireball)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, fireball) in query.iter_mut() {
        transform.translation = transform
            .translation
            .lerp(fireball.target, fireball.speed * delta);

        // info!("Moving fireball to {:?}", transform.translation);
    }
}
