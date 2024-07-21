mod data;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::game::GameState;

pub mod prelude {
    pub use super::data::*;
}

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LevelSelection), spawn_level_selection)
            .add_systems(
                Update,
                (interaction_on_level_selection_buttons).run_if(in_state(GameState::LevelSelection)),
            );
    }
}
