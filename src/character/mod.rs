mod data;
mod systems;

use bevy::prelude::*;
use data::SelectedCharacter;
use systems::*;

use crate::game::GameState;

pub mod prelude {
    pub use super::data::*;
}

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCharacter>()
            .add_systems(
                OnEnter(GameState::CharacterSelection),
                spawn_character_selection,
            )
            .add_systems(
                Update,
                (interaction_on_character_selection_buttons)
                    .run_if(in_state(GameState::CharacterSelection)),
            );
    }
}
