use std::sync::OnceLock;
use bevy::prelude::{Commands, Local};

pub static GLOBAL_CONFIG: OnceLock<InstanceVariables> = OnceLock::new();

pub struct InstanceVariables {
    pub is_client: bool
}

pub struct CurrentState {
    current_state: Box<CurrentState>
}

pub fn is_client() -> bool {
    GLOBAL_CONFIG.get().unwrap().is_client
}

pub fn game_state(commands : Commands, is_client: bool) {
    GLOBAL_CONFIG.set(InstanceVariables { is_client }).unwrap_or_else(|_| {
        println!("SEVERE: Game state variables not initialized!");
    });

}

