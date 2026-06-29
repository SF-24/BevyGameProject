use crate::movement::Velocity;
use bevy::prelude::*;
use bevy::scene::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
}

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_spaceship);
    }
}

// Pass the command queue
fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("spaceship.glb#scene0"));
    commands.spawn((
        Transform::from_translation(STARTING_TRANSLATION),
        // Mesh3d(mesh_assets.add(Sphere::new(1.))),
        // MeshMaterial3d(mat_assets.add(StandardMaterial::default())),
        WorldAssetRoot(model_scene),
        SpaceshipBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
        }
    ));
}