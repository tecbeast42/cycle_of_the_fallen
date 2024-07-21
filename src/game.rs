use bevy::prelude::*;

/// seperate the different phases of the game
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    Startup, // we might not need that, but it is like pre playing
    LevelSelection,
    CharacterSelection,
    Play,
}

/// holds the current level if there is one
#[derive(Resource, Default)]
struct CurrentLevel(Option<usize>);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<CurrentLevel>();
    }
}
