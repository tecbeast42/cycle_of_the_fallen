// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod player;
mod walls;
mod game;
mod levels;

use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use player::prelude::*;
use walls::prelude::*;

fn main() {
    App::new()
        // Disabling gravity
        .insert_resource(Gravity(Vec2::ZERO))
        // Setting global timer for physics update
        .insert_resource(Time::new_with(Physics::fixed_hz(144.0)))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            PhysicsPlugins::default().with_length_unit(PLAYER_RADIUS),
        ))
        .add_plugins(PlayerPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(levels::LevelsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
