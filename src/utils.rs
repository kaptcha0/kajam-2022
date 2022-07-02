use bevy::{
    ecs::{archetype::Archetypes, component::ComponentId},
    prelude::Entity,
};
use heron::PhysicsLayer;

#[allow(dead_code)]
pub fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + (b - a) * x
}

#[allow(dead_code)]
pub fn inv_lerp(x: f32, a: f32, b: f32) -> f32 {
    (x - a) / (b - a)
}

#[allow(dead_code)]
pub fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        if archetype.entities().contains(entity) {
            return Some(archetype.components());
        }
    }
    None
}

#[derive(PhysicsLayer)]
pub enum Layers {
    Player,
    Enemy,
    Level,
}
