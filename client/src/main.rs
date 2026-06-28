use bevy::prelude::*;
use common::game_state::{game_state, InstanceVariables};
use common::spaceship::*;
use common::movement::*;
use common::debug::*;

// Create the main function
fn main() {
    let is_client : bool = true;

    App::new()
        // Create game state and the config
        .add_systems(Startup, move | commands: Commands| {
            game_state(commands, is_client);
        })
        // Init the world
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins).run();
}
