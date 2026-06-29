use crate::game_state::is_client;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;
use bevy::scene::*;
use crate::asset_loader::SceneAssets;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;

#[derive(Component,Debug)]
pub struct Spaceship;

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
}

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,spawn_spaceship);
    }
}

// Pass the command queue
fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    if(is_client()) {
        commands.spawn((
            Transform::from_translation(STARTING_TRANSLATION),
            WorldAssetRoot(scene_assets.spaceship.clone()),
            MovingObjectBundle {
                velocity: Velocity {
                    value: STARTING_VELOCITY,
                },
                acceleration: Acceleration {
                    value: Vec3::ZERO,
                },
            },
            Spaceship,
        ));
    } else {
        commands.spawn((
            Transform::from_translation(STARTING_TRANSLATION),
            MovingObjectBundle {
                velocity: Velocity {
                    value: STARTING_VELOCITY,
                },
                acceleration: Acceleration {
                    value: Vec3::ZERO,
                },
            },
            Spaceship,
        ));
    }
}

fn spaceship_movement_controls(mut commands: Commands, mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>) {
    let transform = query.single();
}