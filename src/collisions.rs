use bevy::prelude::*;
use heron::prelude::*;

use crate::{health::DamageEvent, hud::UpdatePepperCountEvent, player::Player, utils::Layers};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collisions);
    }
}

fn collisions(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut events: EventReader<CollisionEvent>,
    mut pepper_event: EventWriter<UpdatePepperCountEvent>,
    mut damage_event: EventWriter<DamageEvent>,
) {
    for event in events.iter() {
        let data = entities_from_event(event);

        let entities: &[(Entity, Layers); 2] = match data.as_slice().try_into() {
            Ok(data) => data,
            Err(_) => break,
        };

        let player = entities.iter().find(|item| item.1 == Layers::Player);
        let pepper = entities.iter().find(|item| item.1 == Layers::Pepper);
        let fireball = entities.iter().find(|item| item.1 == Layers::Fireball);
        let level = entities.iter().find(|item| item.1 == Layers::Level);
        let enemy = entities.iter().find(|item| item.1 == Layers::Enemy);

        // Interactions with player
        match player {
            Some((player_entity, _)) => {
                let contact = match event {
                    CollisionEvent::Started(_, _) => true,
                    CollisionEvent::Stopped(_, _) => false,
                };

                let mut player = player_query.single_mut();
                player.jumped = !contact;

                match pepper {
                    Some(entity) => {
                        player.peppers += 1;
                        pepper_event.send(UpdatePepperCountEvent(player.peppers));

                        commands.entity(entity.0).despawn_recursive();
                    }
                    None => {}
                }

                match enemy {
                    Some(_) => {
                        for _ in 0..100 {
                            damage_event.send(DamageEvent(*player_entity));
                        }
                    }
                    None => {}
                }
            }
            None => {}
        };

        match level {
            Some(_) => match fireball {
                Some((entity, _)) => commands.entity(*entity).despawn_recursive(),
                None => {}
            },
            None => {}
        }

        // Interactions with enemy
        match enemy {
            Some((entity, _)) => match fireball {
                Some((fb, _)) => {
                    damage_event.send(DamageEvent(*entity));
                    commands.entity(*fb).despawn_recursive();
                }
                None => {}
            },
            None => {}
        };
    }
}

fn entities_from_event(ev: &CollisionEvent) -> Vec<(Entity, Layers)> {
    let layers = ev.collision_layers();
    let rigid_bodies = ev.rigid_body_entities();
    let mut entities = Vec::new();

    let queries = [
        (has_group(layers, Layers::Level), Layers::Level),
        (has_group(layers, Layers::Fireball), Layers::Fireball),
        (has_group(layers, Layers::Enemy), Layers::Enemy),
        (has_group(layers, Layers::Pepper), Layers::Pepper),
        (has_group(layers, Layers::Player), Layers::Player),
    ];

    for (query, layer) in queries.iter() {
        match query {
            Some(num) => {
                if *num {
                    entities.push((rigid_bodies.0, *layer))
                } else {
                    entities.push((rigid_bodies.1, *layer))
                }
            }
            None => {}
        }
    }

    entities
}

fn has_group(layers: (CollisionLayers, CollisionLayers), target_layer: Layers) -> Option<bool> {
    let (l1, l2) = layers;

    if l1.contains_group(target_layer) {
        return Some(true);
    }

    if l2.contains_group(target_layer) {
        return Some(false);
    }

    return None;
}
