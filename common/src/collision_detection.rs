use crate::asteroids::Asteroid;
use crate::schedule::InGameSet;
use crate::spaceship::Spaceship;
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component,Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,collision_detection.in_set(InGameSet::CollisionDetection))
            .add_systems(Update,
            (
                    handle_collisions::<Asteroid>,
                    handle_collisions::<Spaceship>
                ).in_set(InGameSet::CollisionDetection)
            );
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // Detect the collisions.
    for(entity_a, transform_a, collider_a) in query.iter() {
        for(entity_b,transform_b,collider_b) in query.iter() {
            if(entity_a != entity_b) {
                let distance = transform_a.translation().distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    // Save collision.
                    colliding_entities.entry(entity_a).or_insert_with(Vec::new).push(entity_b);
                }
            }
        }
    }

    // Update colliders. Clear colliding entities from previous phase and extend colliding entities.
    for(entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend(collisions.iter());
        }
    }
}

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for collided_entity in collider.colliding_entities.iter() {
            // If collided with the same type, continue.
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Despawn
            commands.entity(entity).try_despawn();
        }
    }
}

