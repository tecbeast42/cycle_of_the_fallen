use crate::game::GameState;

use super::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

/// Spawn the walls to the map.
///
/// Allow to spawn all the walls of the current level.
pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let level_height = 600.0;
    let level_width = 1000.0;

    // Left wall
    commands.spawn((
        Wall,
        StateScoped(GameState::Play),
        ColorMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::new(WALL_WIDTH, level_height)))
                .into(),
            material: materials.add(Color::linear_rgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(level_width / -2.0 - WALL_WIDTH / 2.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WALL_WIDTH, level_height),
    ));

    // Right wall
    commands.spawn((
        Wall,
        ColorMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::new(WALL_WIDTH, level_height)))
                .into(),
            material: materials.add(Color::linear_rgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(level_width / 2.0 + WALL_WIDTH / 2.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WALL_WIDTH, level_height),
    ));

    // Top wall
    commands.spawn((
        Wall,
        ColorMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::new(
                    level_width + WALL_WIDTH * 2.0,
                    WALL_WIDTH,
                )))
                .into(),
            material: materials.add(Color::linear_rgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(0.0, level_height / -2.0 - WALL_WIDTH / 2.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(level_width + WALL_WIDTH * 2.0, WALL_WIDTH),
    ));

    // Bottom wall
    commands.spawn((
        Wall,
        ColorMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::new(
                    level_width + WALL_WIDTH * 2.0,
                    WALL_WIDTH,
                )))
                .into(),
            material: materials.add(Color::linear_rgb(0.3, 0.3, 0.3)),
            transform: Transform::from_xyz(0.0, level_height / 2.0 + WALL_WIDTH / 2.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(level_width + WALL_WIDTH * 2.0, WALL_WIDTH),
    ));
}

pub fn spawn_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle =
        asset_server.load("kennys-sokoban/kenney_sokoban-pack/PNG/Retina/Ground/ground_04.png");
    for x in 0..10 {
        for y in 0..6 {
            commands.spawn(SpriteBundle {
                texture: handle.clone(),
                transform: Transform::from_xyz(
                    x as f32 * 100.0 - 450.0,
                    y as f32 * 100.0 - 250.0,
                    -1.0,
                ),
                ..default()
            });
        }
    }
}
