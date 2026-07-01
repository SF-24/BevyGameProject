use crate::asset_loader::SceneAssets;
use crate::game_state::is_client;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;
use bevy::scene::*;
use bevy::window::WindowEvent::KeyboardInput;
use crate::collision_detection::Collider;
use crate::schedule::InGameSet;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_SCALE : f32 = 0.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const MISSILE_SPEED: f32 = 20.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 4.0;
const MISSILE_RADIUS: f32 = 1.0;

#[derive(Component,Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component,Debug)]
pub struct SpaceshipMissile;

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
}

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,spawn_spaceship)
            .add_systems(Update,(spaceship_movement_controls,spaceship_weapon_controls,spaceship_shield_controls).chain().in_set(InGameSet::UserInput));
    }
}

// Pass the command queue
fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    if(is_client()) {
        commands.spawn((
            Transform::from_translation(STARTING_TRANSLATION).with_scale(Vec3::splat(SPACESHIP_SCALE)),
            WorldAssetRoot(scene_assets.spaceship.clone()),
            MovingObjectBundle {
                velocity: Velocity {
                    value: STARTING_VELOCITY,
                },
                acceleration: Acceleration {
                    value: Vec3::ZERO,
                },
                collider: Collider::new(SPACESHIP_RADIUS),
            },
            Spaceship,
        ));
    } else {
        commands.spawn((
            Transform::from_translation(STARTING_TRANSLATION).with_scale(Vec3::splat(SPACESHIP_SCALE)),
            MovingObjectBundle {
                velocity: Velocity {
                    value: STARTING_VELOCITY,
                },
                acceleration: Acceleration {
                    value: Vec3::ZERO,
                },
                collider: Collider::new(SPACESHIP_RADIUS),
            },
            Spaceship,
        ));
    }
}

fn spaceship_movement_controls(mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
   time: Res<Time>
) {
    let Ok((mut transform, mut velocity)) = query.single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // Rotation. Apply delta secs, as it isn't applied earlier (e.g. w. velocity)
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED*time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED*time.delta_secs();
    }

    // Direction
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }

    if(keyboard_input.pressed(KeyCode::KeyQ)) {
        roll = -SPACESHIP_ROLL_SPEED*time.delta_secs();
    } else if(keyboard_input.pressed(KeyCode::KeyE)) {
        roll = SPACESHIP_ROLL_SPEED*time.delta_secs();
    }

    // Rotate around the global y-axis.
    // If the camera follows the spaceship, make this global.
    transform.rotate_y(rotation);

    // Rotate around the local z-axis
    transform.rotate_local_z(roll);

    // Update velocity
    // Negate due to bevy having reversed z direction convention.
    // Left hand vs right hand axis.
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(mut commands: Commands, query: Query<&Transform, With<Spaceship>>,
     keyboard_input: Res<ButtonInput<KeyCode>>,
     scene_assets: Res<SceneAssets>
) {
    let Ok((transform)) = query.single() else {
        return;
    };
    if(keyboard_input.pressed(KeyCode::Space)) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
            },
            Transform::from_translation(
                transform.translation -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR
            ),
            WorldAssetRoot(scene_assets.missiles.clone()),
            SpaceshipMissile,
        ));
    }
}

fn spaceship_shield_controls(mut commands: Commands, query: Query<Entity, With<Spaceship>>,
                             keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(spaceship) = query.single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::KeyR) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}