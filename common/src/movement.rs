use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Component, Query, Transform};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

#[derive(Component,Debug)]
pub struct Velocity {
    pub value: Vec3
}


// Update positions for all entities
fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.translation.x += velocity.value.x;
        position.translation.y += velocity.value.y;
        position.translation.z += velocity.value.z;
    }
}