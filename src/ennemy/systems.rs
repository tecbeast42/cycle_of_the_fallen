use crate::{
    game::{CurrentLevel, GameState},
    AttackProjectile,
};

use super::prelude::*;
use crate::player::prelude::*;
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
                StateScoped(GameState::Play),
                EnemyBundle::new(kind, radius, &mut meshes, &mut materials),
            ));
        }
        2 => {
            let kind = EnnemyKind::Turret;
            let radius = kind.radius();
            commands.spawn((
                StateScoped(GameState::Play),
                AlwaysAttack,
                AttackSpeed::from_type(AttackSpeedType::Regular),
                EnemyBundle::new(kind, radius, &mut meshes, &mut materials),
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
    player_query: Query<(&Transform, &Team), With<Targetable>>,
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
        for (player_transform, team) in player_query.iter() {
            if team != &Team::Enemy {
                continue;
            }
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
            let direction = (target - projectile_transform.translation.truncate()).normalize();
            projectile_transform.translation += direction.extend(0.0) * 23.0;
            let velocity = direction * 200.0;
            // attack
            commands.spawn((
                Name::new("Enemy Projectile"),
                StateScoped(GameState::Play),
                Team::Enemy,
                AttackProjectile::new(projectile_transform.translation.truncate(), 3000.0),
                ColorMesh2dBundle {
                    mesh: meshes.add(Circle::new(7.0)).into(),
                    material: materials.add(Color::from(tailwind::PINK_400)),
                    transform: projectile_transform,
                    ..default()
                },
                RigidBody::Dynamic,
                LinearVelocity(velocity),
                Collider::circle(7.0),
                get_collision_layers(ENEMY_PROJECTILE_COLLISION_LAYER),
            ));
        }
    }
}
