mod data;
mod events;
mod systems;

use bevy::prelude::*;
use data::*;
use events::*;
use systems::*;

use crate::game::CurrentLevel;
use crate::game::GameState;
use crate::player::prelude::*;

pub mod prelude {
    pub use super::data::*;
    pub use super::events::*;
    pub use super::LevelHistoryPlugin;
}

pub struct LevelHistoryPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelHistorySet {
    Replay,
    Record,
    Clear,
    Debug,
    SavePlayer,
    SpawnGhost,
}

fn level_changed(current_level: Res<CurrentLevel>) -> bool {
    current_level.is_changed()
}

impl Plugin for LevelHistoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SavePlayerGhostEvent>()
            .init_resource::<LevelStartTime>()
            .init_resource::<PlayerGhostList>()
            .configure_sets(
                Update,
                LevelHistorySet::Clear
                    .after(LevelHistorySet::Record)
                    .run_if(level_changed),
            )
            .configure_sets(OnEnter(GameState::LevelSelection), LevelHistorySet::Clear)
            .configure_sets(
                Update,
                (
                    LevelHistorySet::Replay,
                    LevelHistorySet::Record,
                    LevelHistorySet::Clear,
                    LevelHistorySet::Debug,
                )
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                OnEnter(GameState::Play),
                (
                    spawn_ghosts.in_set(LevelHistorySet::SpawnGhost),
                    reset_level_start_time,
                ),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                save_player_ghost.in_set(LevelHistorySet::SavePlayer),
            )
            .add_systems(Update, (save_player_ghost, clean_ghost_list))
            .init_resource::<LevelHistory<PlayerMoveEvent>>()
            .add_systems(
                Update,
                (
                    record_event::<PlayerMoveEvent>.in_set(LevelHistorySet::Record),
                    replay_event::<PlayerMoveEvent>.in_set(LevelHistorySet::Replay),
                    clear_history::<PlayerMoveEvent>.in_set(LevelHistorySet::Clear),
                    debug_history::<PlayerMoveEvent>.in_set(LevelHistorySet::Debug),
                ),
            )
            .init_resource::<LevelHistory<PlayerRotateEvent>>()
            .add_systems(
                Update,
                (
                    record_event::<PlayerRotateEvent>.in_set(LevelHistorySet::Record),
                    replay_event::<PlayerRotateEvent>.in_set(LevelHistorySet::Replay),
                    clear_history::<PlayerRotateEvent>.in_set(LevelHistorySet::Clear),
                    debug_history::<PlayerRotateEvent>.in_set(LevelHistorySet::Debug),
                ),
            )
            .init_resource::<LevelHistory<PlayerAttackEvent>>()
            .add_systems(
                Update,
                (
                    record_event::<PlayerAttackEvent>.in_set(LevelHistorySet::Record),
                    replay_event::<PlayerAttackEvent>.in_set(LevelHistorySet::Replay),
                    clear_history::<PlayerAttackEvent>.in_set(LevelHistorySet::Clear),
                    debug_history::<PlayerAttackEvent>.in_set(LevelHistorySet::Debug),
                ),
            );
    }
}
