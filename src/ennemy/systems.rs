use crate::{
    game::{CurrentLevel, GameState},
    AttackProjectile, Ghost, Player,
};

use super::prelude::*;
use avian2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

/// Spawn ennemy to the map.
///
/// For now the ennemy is spawned to the right of the level.
pub fn spawn_ennemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_level: Res<CurrentLevel>,
) {
    let Some(level) = &current_level.0 else {
        return;
    };

    match level.id {
        1 => {
            let kind = EnnemyKind::Dummy;
            let radius = kind.radius();
            commands.spawn((
                Ennemy,
                StateScoped(GameState::Play),
                kind,
                ColorMesh2dBundle {
                    mesh: meshes.add(Circle::new(radius)).into(),
                    material: materials.add(Color::linear_rgb(0.6, 0.2, 0.1)),
                    transform: Transform::from_xyz(300.0, 100.0, 0.0),
                    ..default()
                },
                RigidBody::Static,
                Collider::circle(radius),
            ));
        }
        2 => {
            let kind = EnnemyKind::Turret;
            let radius = kind.radius();
            commands.spawn((
                Ennemy,
                StateScoped(GameState::Play),
                AttackSpeed::new(Timer::from_seconds(2.0, TimerMode::Once)),
                AlwaysAttack,
                EnnemyKind::Turret,
                ColorMesh2dBundle {
                    mesh: meshes.add(Circle::new(radius)).into(),
                    material: materials.add(Color::linear_rgb(0.6, 0.2, 0.1)),
                    transform: Transform::from_xyz(300.0, 100.0, 0.0),
                    ..default()
                },
                RigidBody::Static,
                Collider::circle(radius),
            ));
        }
        _ => {}
    }
}

/// just process time for all [`AttackSpeed`] components
pub fn tick_attack_speed(time: Res<Time>, mut query: Query<&mut AttackSpeed, With<Ennemy>>) {
    for mut attack_speed in query.iter_mut() {
        attack_speed.tick(time.delta());
    }
}

pub fn execute_always_attack(
    mut ennemy_query: Query<(&mut AttackSpeed, &Transform), (With<Ennemy>, With<AlwaysAttack>)>,
    player_query: Query<&Transform, Or<(With<Ghost>, With<Player>)>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut attack_seed, ennemy_tranform) in ennemy_query.iter_mut() {
        if !attack_seed.finished() {
            continue;
        }
        // we can attack now
        attack_seed.reset();

        let mut distance: f32 = f32::MAX;
        let mut target: Vec2 = Vec2::ZERO;
        for player_transform in player_query.iter() {
            let test_distance = ennemy_tranform
                .translation
                .distance_squared(player_transform.translation);
            if test_distance < distance {
                target = player_transform.translation.truncate();
                distance = test_distance;
            }
        }

        // check if there is something to attack
        if distance < f32::MAX {
            let mut projectile_transform = ennemy_tranform.to_owned();
            projectile_transform.translation += Vec3::X * 20.0;
            projectile_transform.look_at(target.extend(0.0), Vec3::Z);
            // attack
            commands.spawn((
                AttackProjectile::new(projectile_transform.translation.truncate(), 3000.0),
                ColorMesh2dBundle {
                    mesh: meshes.add(Rectangle::new(7.0, 16.0)).into(),
                    material: materials.add(Color::from(tailwind::PINK_400)),
                    transform: projectile_transform,
                    ..default()
                },
                RigidBody::Dynamic,
                LinearVelocity(Vec2::X * 125.0),
                Collider::rectangle(7.0, 16.0),
            ));
        }
    }
}
