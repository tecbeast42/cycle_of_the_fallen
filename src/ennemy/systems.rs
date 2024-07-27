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
    asset_server: Res<AssetServer>,
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
                EnemyBundle::new(kind, radius),
                SpriteBundle {
                    transform: Transform::from_xyz(300.0, 25.0, 2.0),
                    texture: asset_server.load("kenney_tower/PNG/Retina/towerDefense_tile249b.png"),
                    ..default()
                },
            ));
        }
        2 => {
            let kind = EnnemyKind::Turret;
            let radius = kind.radius();
            commands.spawn((
                StateScoped(GameState::Play),
                AttackSpeed::from_type(AttackSpeedType::Regular),
                EnemyBundle::new(kind, radius),
                SpriteBundle {
                    transform: Transform::from_xyz(300.0, 25.0, 2.0),
                    texture: asset_server.load("kenney_tower/PNG/Retina/towerDefense_tile249.png"),
                    ..default()
                },
            ));
        }
        3 => {
            let kind = EnnemyKind::Turret;
            let radius = kind.radius();
            commands.spawn((
                StateScoped(GameState::Play),
                AttackSpeed::from_type(AttackSpeedType::Regular),
                EnemyBundle::new(kind, radius),
                SpriteBundle {
                    transform: Transform::from_xyz(300.0, 25.0, 2.0),
                    texture: asset_server.load("kenney_tower/PNG/Retina/towerDefense_tile249.png"),
                    ..default()
                },
            ));
            commands.spawn((
                StateScoped(GameState::Play),
                AttackSpeed::from_type(AttackSpeedType::Regular),
                EnemyBundle::new(kind, radius),
                SpriteBundle {
                    transform: Transform::from_xyz(300.0, -25.0, 2.0),
                    texture: asset_server.load("kenney_tower/PNG/Retina/towerDefense_tile249.png"),
                    ..default()
                },
            ));
        },
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
    mut ennemy_query: Query<(Option<&mut AttackSpeed>, &mut Transform), With<Ennemy>>,
    player_query: Query<&Transform, (Or<(With<Ghost>, With<Player>)>, Without<Ennemy>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (option_attack_speed, mut ennemy_tranform) in ennemy_query.iter_mut() {
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
        let ennemy_position = ennemy_tranform.translation.truncate();
        let direction = target - ennemy_position;
        let angle = direction.y.atan2(direction.x);
        ennemy_tranform.rotation = Quat::from_rotation_z(angle);

        let Some(mut attack_speed) = option_attack_speed else {
            continue;
        };

        if !attack_speed.finished() {
            continue;
        }
        // we can attack now
        attack_speed.reset();

        // check if there is something to attack
        if distance < f32::MAX {
            let mut projectile_transform = ennemy_tranform.to_owned();
            let direction = (target - projectile_transform.translation.truncate()).normalize();
            projectile_transform.translation += direction.extend(5.0) * 23.0;
            let velocity = direction * 200.0;
            // attack
            commands.spawn((
                StateScoped(GameState::Play),
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
                CollisionLayers::from_bits(0b0010, 0b0110)
            ));
        }
    }
}
