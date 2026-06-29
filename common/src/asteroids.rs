use crate::asset_loader::SceneAssets;
use crate::game_state::is_client;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;
use rand::RngExt;
use std::ops::Range;

const VELOCITY_SCALAR: f32 = 5.0;
const DEFAULT_SCALE: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

#[derive(Component,Debug)]
pub struct Asteroid;

#[derive(Resource,Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating)
        }).add_systems(Update,spawn_asteroid);
    }
}

fn spawn_asteroid(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, scene_assets: Res<SceneAssets>) {
    spawn_timer.timer.tick(time.delta());

    if(!spawn_timer.timer.just_finished()) {
        return;
    }
    let mut rng = rand::rng(); // was thread_rng
    let translation = Vec3::new(rng.random_range(SPAWN_RANGE_X),0.,rng.random_range(SPAWN_RANGE_Z));

    let mut random_unit_vector = || Vec3::new(rng.random_range(-1.0..1.0),0.,rng.random_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector();
    let acceleration = random_unit_vector();

    // Spawn
    if(is_client()) {
        commands.spawn((
            WorldAssetRoot(scene_assets.asteroid.clone()),
            Transform::from_translation(translation),
            MovingObjectBundle {
                velocity: Velocity::new(velocity),
                acceleration: Acceleration::new(acceleration),
            },
            Asteroid,
        ));
    } else {
        commands.spawn((
            Transform::from_translation(translation),
            Transform::from_scale(Vec3::splat(DEFAULT_SCALE)),
            MovingObjectBundle {
                velocity: Velocity::new(velocity),
                acceleration: Acceleration::new(acceleration),
            },
            Asteroid,
        ));
    }
}