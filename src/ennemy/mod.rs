mod data;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::game::GameState;

pub mod prelude {
    pub use super::data::*;
    pub use super::EnnemyPlugin;
    pub use crate::collision::*;
}

pub struct EnnemyPlugin;

impl Plugin for EnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), spawn_ennemies)
            .add_systems(
                Update,
                (tick_attack_speed, execute_always_attack).run_if(in_state(GameState::Play)),
            );
    }
}
