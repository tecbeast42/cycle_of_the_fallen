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
