use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Component, Query, Res, Transform};
use bevy::time::Time;

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
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.value.x * time.delta_secs();
        transform.translation.y += velocity.value.y * time.delta_secs();
        transform.translation.z += velocity.value.z * time.delta_secs();
    }
}