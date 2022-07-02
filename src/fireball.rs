use bevy::{prelude::*, render::camera::RenderTarget};
use heron::{Acceleration, CollisionLayers, CollisionShape, RigidBody, Velocity};

use crate::{
    assets::GameAssets,
    player::{Player, PLAYER_SIZE},
    utils::Layers,
};

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnFireballEvent>()
            .add_system(create_fireball)
            .add_system(spawn_fireballs)
            .add_system(despawn_fireball);
    }
}

pub struct SpawnFireballEvent {
    spawn_point: Vec3,
    target: Vec3,
}

#[derive(Component)]
pub struct Fireball;

fn create_fireball(
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<&Transform, With<Player>>,
    mut event_writer: EventWriter<SpawnFireballEvent>,
) {
    if mouse.pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Left) {
        let player = player_query.single();

        if let Some(position) = get_world_coords(&windows, &q_camera) {
            let ev = SpawnFireballEvent {
                spawn_point: player.translation,
                target: position.extend(900.0),
            };

            event_writer.send(ev);
        }
    }
}

/// Translates mouse coordinates to world space
fn get_world_coords(
    wnds: &Res<Windows>,
    q_camera: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        return Some(world_pos);
    }

    return None;
}

fn spawn_fireballs(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnFireballEvent>,
) {
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

        let dist = ev
            .target
            .truncate()
            .distance(bundle.transform.translation.truncate());

        let impulse = dist * 5.0;
        let dy = ev.target.y - bundle.transform.translation.y;
        let dx = ev.target.x - bundle.transform.translation.x;
        let angle = f32::atan2(dy, dx);

        bundle.transform.rotation = Quat::from_rotation_z(angle);

        let fireball_vec = Vec3::new(angle.cos() * impulse, angle.sin() * impulse, 900.0);

        commands
            .spawn_bundle(bundle)
            .insert(RigidBody::Dynamic)
            .insert(Velocity::from_linear(fireball_vec))
            .insert(Acceleration::default())
            .insert(CollisionShape::Capsule {
                half_segment: (PLAYER_SIZE),
                radius: (PLAYER_SIZE * 2.0),
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layers::Fireball)
                    .with_masks(&[Layers::Player, Layers::Enemy]),
            )
            .insert(Fireball)
            .insert(Name::new("fireball"));
    }
}

fn despawn_fireball(mut commands: Commands, query: Query<(Entity, &Transform), With<Fireball>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -1.5 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
