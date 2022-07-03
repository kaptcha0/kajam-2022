use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{hud::UpdateHealthPointsEvent, player::Player};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_system(damage_enemy)
            .add_system(damage_player);
    }
}

pub struct DamageEvent(pub Entity);

#[derive(Component, Inspectable)]
pub struct Health(pub f32);

fn damage_enemy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health), Without<Player>>,
    mut events: EventReader<DamageEvent>,
) {
    let health_damage = 0.05;

    for ev in events.iter() {
        let item = query.iter_mut().find(|item| item.0 == ev.0);

        if let Some((entity, mut health)) = item {
            if health.0 <= 0.0 {
                commands.entity(entity).despawn_recursive();
            }

            health.0 -= health_damage;
        }
    }
}

fn damage_player(
    _commands: Commands,
    mut query: Query<(Entity, &mut Health), With<Player>>,
    mut events: EventReader<DamageEvent>,
    mut writer: EventWriter<UpdateHealthPointsEvent>,
) {
    let health_damage = 0.1;

    for ev in events.iter() {
        let item = query.iter_mut().find(|item| item.0 == ev.0);

        if let Some((_entity, mut health)) = item {
            if health.0 <= 0.0 {
                todo!("add game over screen");
            }

            health.0 -= health_damage;
            writer.send(UpdateHealthPointsEvent(health.0));
        }
    }
}
