mod data;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::game::GameState;

pub mod prelude {
    pub use super::data::*;
    pub use super::WallPlugin;
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), (spawn_walls, spawn_floor));
    }
}
