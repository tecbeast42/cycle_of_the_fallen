use super::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

/// Spawn ennemy to the map.
///
/// For now the ennemy is spawned to the right of the level.
pub fn spawn_ennemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Ennemy,
        EnnemyStats::new(EnnemyType::Dummy),
        ColorMesh2dBundle {
            mesh: meshes.add(Circle::new(TURRET_RADIUS)).into(),
            material: materials.add(Color::linear_rgb(0.6, 0.2, 0.1)),
            transform: Transform::from_xyz(300.0, 100.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::circle(TURRET_RADIUS),
    ));
}
