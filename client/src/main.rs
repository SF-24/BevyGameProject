mod camera;

use crate::camera::CameraPlugin;
use bevy::prelude::*;
use common::asset_loader::AssetLoaderPlugin;
use common::asteroids::AsteroidsPlugin;
use common::collision_detection::CollisionDetectionPlugin;
use common::despawn_plugin::DespawnPlugin;
use common::game_state::game_state;
use common::movement::*;
use common::schedule::SchedulePlugin;
use common::spaceship::*;

// Create the main function
fn main() {
    let is_client : bool = true;
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1,0.0,0.15)))
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(1.0,1.0,1.0),
            brightness: 1000.0, // usually 500-5000
            affects_lightmapped_meshes: true,
        })
        // Create game state and the config
        .add_systems(Startup,  move | commands: Commands| {
            game_state(commands, is_client);
        })
        // Init the world
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin) // Bevy scheduler.
        .run();

}
