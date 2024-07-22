use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

pub fn toggle_game_over(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if state.get() == &GameState::Playing {
        next_state.set(GameState::GameOver);
    } else if state.get() == &GameState::GameOver {
        next_state.set(GameState::Playing);
    }
}
