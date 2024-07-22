mod data;
mod systems;

use avian2d::schedule::PostProcessCollisions;
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
                    despawn_out_of_range_projectiles,
                    attack,
                )
                    .chain(),
            )
            .add_systems(PostProcessCollisions, despawn_collided_projectiles);
    }
}
