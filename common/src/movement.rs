use crate::collision_detection::Collider;
use crate::schedule::InGameSet;
use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Bundle, Component, IntoScheduleConfigs, Query, Res, Transform};
use bevy::time::Time;

// Components:
#[derive(Component,Debug)]
pub struct Velocity {
    pub value: Vec3
}
impl Velocity {pub fn new(value: Vec3) -> Self {Self { value }}}

#[derive(Component,Debug)]
pub struct Acceleration {
    pub value: Vec3,
}
impl Acceleration {pub fn new(value: Vec3) -> Self {Self { value }}}

// Bundle
#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
}

// Plugin definition:
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position,update_velocity).chain().in_set(InGameSet::EntityUpdates));
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for(acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_secs();
    }
}

// Update positions for all entities
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_secs();
    }
}