// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use cycle_of_the_fallen::character::*;
use cycle_of_the_fallen::level::history::*;
use cycle_of_the_fallen::level::state::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            PhysicsPlugins::default(),
        ))
        .insert_state(GameState::Playing)
        .init_resource::<PlayerGhostList>()
        .init_resource::<LevelHistory<CharacterMoveEvent>>()
        .init_resource::<LevelStartTime>()
        .add_event::<CharacterMoveEvent>()
        .add_event::<GameOverEvent>()
        .add_systems(Startup, setup_camera)
        .add_systems(
            OnEnter(GameState::Playing),
            (reset_game_history_start, spawn_character, spawn_ghosts),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            (character_kill, ghost_despawn),
        )
        .add_systems(
            Update,
            (
                character_move_input,
                character_move_read,
                record_event::<CharacterMoveEvent>,
                replay_event::<CharacterMoveEvent>,
                debug_history::<CharacterMoveEvent>,
                toggle_game_over
                    .run_if(|keys: Res<ButtonInput<KeyCode>>| keys.just_pressed(KeyCode::KeyK)),
                // character_kill
                //     .run_if(|keys: Res<ButtonInput<KeyCode>>| keys.just_pressed(KeyCode::KeyK)),
                // spawn_ghosts
                //     .run_if(|keys: Res<ButtonInput<KeyCode>>| keys.just_pressed(KeyCode::KeyS)),
                // spawn_character
                //     .run_if(|keys: Res<ButtonInput<KeyCode>>| keys.just_pressed(KeyCode::KeyS)),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
