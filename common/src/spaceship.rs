use bevy::app::{App, Plugin, Startup};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Transform};
use bevy::scene::ResolvedSceneRoot;
use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

struct SpaceshipBundle {
    velocity: Velocity,
    model: ResolvedSceneRoot
}

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_spaceship);
    }
}

// Pass the command queue
fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
        Velocity {
            value: Vec3::new(0.,0.,0.)
        }
    ));
}