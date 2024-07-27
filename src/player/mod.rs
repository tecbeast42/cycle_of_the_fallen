mod data;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::game::GameState;

pub mod prelude {
    pub use super::data::*;
    pub use super::PlayerPlugin;
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), spawn_player)
            .add_systems(
                Update,
                (
                    move_player,
                    rotate_player,
                    player_attack,
                    check_for_level_complete,
                    handle_projectile_colissions,
                    despawn_out_of_range_projectiles,
                )
                    .run_if(in_state(GameState::Play)),
            );
    }
}
