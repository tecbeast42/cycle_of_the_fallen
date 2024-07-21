mod data;
mod systems;

use bevy::prelude::*;
use systems::*;

pub mod prelude {
    pub use super::data::*;
    pub use super::PlayerPlugin;
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, rotate_player, attack).chain());
    }
}
