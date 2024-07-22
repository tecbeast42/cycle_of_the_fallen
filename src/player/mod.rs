mod data;
mod events;
mod systems;

use avian2d::schedule::PostProcessCollisions;
use bevy::prelude::*;
use events::*;
use systems::*;

use crate::game::GameState;
use crate::level_history::LevelHistorySet;

pub mod prelude {
    pub use super::data::*;
    pub use super::events::*;
    pub use super::PlayerPlugin;
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMoveEvent>()
            .add_event::<PlayerRotateEvent>()
            .add_event::<PlayerAttackEvent>()
            .add_systems(OnEnter(GameState::Play), spawn_player)
            .add_systems(
                OnEnter(GameState::GameOver),
                despawn_player.after(LevelHistorySet::SavePlayer),
            )
            .add_systems(OnEnter(GameState::LevelSelection), despawn_player)
            .add_systems(OnEnter(GameState::CharacterSelection), despawn_player)
            .add_systems(
                Update,
                (
                    move_player_write,
                    move_player_read,
                    rotate_player_write,
                    rotate_player_read,
                    player_attack_write,
                    player_attack_read,
                    check_for_level_complete,
                    despawn_out_of_range_projectiles,
                )
                    .chain()
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(PostProcessCollisions, despawn_collided_projectiles);
    }
}
