use crate::Class;
use bevy::prelude::*;

/// seperate the different phases of the game
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    LevelSelection,
    CharacterSelection,
    Play,
    GameOver,
}

/// holds the current level if there is one
#[derive(Resource, Default)]
pub struct CurrentLevel(pub Option<Level>);

#[derive(Clone)]
pub struct Level {
    pub id: usize,
    /// is it available to play
    pub unlocked: bool,
    /// the score for the level
    pub cycles: Option<usize>,
    /// available characters,
    pub characters: Vec<Class>,
}

/// holds the current level if there is one
#[derive(Resource, Deref, DerefMut)]
pub struct Levels(Vec<Level>);
impl Levels {
    pub fn id(&self, id: usize) -> Option<Level> {
        for level in self.0.iter() {
            if level.id == id {
                return Some(level.clone());
            }
        }

        panic!("Could not get the right level from Levels");
    }
}

impl Default for Levels {
    fn default() -> Self {
        Self(vec![
            Level {
                id: 1,
                unlocked: true,
                cycles: None,
                characters: vec![Class::Knight, Class::Ranger, Class::Wizard], // Temporarly all classes for test purposes
            },
            Level {
                id: 2,
                unlocked: false,
                cycles: None,
                characters: vec![Class::Knight, Class::Wizard],
            },
        ])
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .enable_state_scoped_entities::<GameState>()
            .init_resource::<CurrentLevel>()
            .init_resource::<Levels>()
            .add_systems(
                Update,
                debug_game_over.run_if(|keyboard_input: Res<ButtonInput<KeyCode>>| {
                    keyboard_input.just_pressed(KeyCode::KeyK)
                }),
            );
    }
}

fn debug_game_over(state: Res<State<GameState>>, mut next_state: ResMut<NextState<GameState>>) {
    if state.get() == &GameState::GameOver {
        next_state.set(GameState::LevelSelection);
    } else if state.get() == &GameState::Play {
        next_state.set(GameState::GameOver);
    }
}
