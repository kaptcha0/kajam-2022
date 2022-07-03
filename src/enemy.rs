use crate::{
    block_type::{BLOCK_HEIGHT, BLOCK_SIZE},
    health::Health,
    player::PLAYER_SIZE,
    terrain::MAP_LEN,
    utils::Layers,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemies);
    }
}

#[derive(Component)]
pub struct Enemy;

fn spawn_enemies(mut commands: Commands) {
    let enemy_count = MAP_LEN / 25;

    for i in 0..enemy_count {
        commands
            .spawn_bundle(load_enemy_entity(i as f32, enemy_count as f32))
            .insert(Health(75.0))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::from_linear(Vec3::X * 0.0))
            .insert(Acceleration::default())
            .insert(CollisionShape::Sphere {
                radius: PLAYER_SIZE,
            })
            .insert(PhysicMaterial {
                restitution: 0.35,
                friction: 0.0,
                density: PLAYER_SIZE * 5.0,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layers::Enemy)
                    .with_masks(&[Layers::Player, Layers::Level, Layers::Fireball]),
            )
            .insert(Enemy);
    }
}

fn load_enemy_entity(index: f32, count: f32) -> impl Bundle {
    let shape = shapes::RegularPolygon {
        sides: 20,
        feature: shapes::RegularPolygonFeature::Radius(PLAYER_SIZE / 2.0),
        ..shapes::RegularPolygon::default()
    };
    let mut rng = rand::thread_rng();
    let range = (
        Vec2::splat(BLOCK_SIZE * 5.0),
        Vec2::splat((MAP_LEN as f32 * BLOCK_SIZE) - (BLOCK_SIZE * 5.0)),
    );

    let noise = rng.gen::<f32>() * 10.0;
    let x = range.0.lerp(range.1, index / count).x + (noise * BLOCK_SIZE);

    let loc = Vec3::new(x, 0.75, 850.0);

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::CYAN, BLOCK_HEIGHT),
        },
        Transform {
            translation: loc,
            ..default()
        },
    )
}
