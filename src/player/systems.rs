use crate::game::{CurrentLevel, GameState, Levels};
use crate::level_history::prelude::*;
use crate::EnnemyStats;
use crate::{character::prelude::SelectedCharacter, Ennemy};

use super::prelude::*;
use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

/// Spawn the player to the map.
///
/// For now the player is spawned to the leftof the level.
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    selected_character: Res<SelectedCharacter>,
) {
    commands
        .spawn((
            Player,
            StateScoped(GameState::Play),
            PlayerStats::new(selected_character.0.clone()),
            ColorMesh2dBundle {
                mesh: meshes.add(Circle::new(PLAYER_RADIUS)).into(),
                material: materials.add(Color::linear_rgb(0.2, 0.5, 0.2)),
                transform: Transform::from_xyz(-400.0, 0.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::circle(PLAYER_RADIUS),
        ))
        // This is temporary it allows to see were is the player facing
        // (Either with delete this, or we use it to insert a sprite for the weapon
        // so it is independant from the player, like in Brotato)
        .with_children(|player| {
            player.spawn(ColorMesh2dBundle {
                mesh: meshes.add(Rectangle::new(8.0, 8.0)).into(),
                material: materials.add(Color::linear_rgb(0.6, 0.2, 0.2)),
                transform: Transform::from_xyz(12.0, 0.0, 1.0),
                ..default()
            });
        });
}

pub fn despawn_player(mut commands: Commands, query_player: Query<Entity, With<Player>>) {
    for entity in query_player.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Moves the player around.
///
/// Use ZQSD (or WASD) to move the player around.
pub fn move_player_write(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut events: EventWriter<PlayerMoveEvent>,
) {
    let delta_time = time.delta_seconds();

    let mut direction = Vec2::ZERO;

    for entity in &mut player {
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        direction = direction.normalize_or_zero() * delta_time * 10000.0;

        events.send(PlayerMoveEvent {
            delta: direction,
            source: EventSource::Input,
            entity,
        });
    }
}

pub fn move_player_read(
    mut player: Query<&mut LinearVelocity>,
    mut events: EventReader<PlayerMoveEvent>,
) {
    for event in events.read() {
        if let Ok(mut velocity) = player.get_mut(event.entity) {
            velocity.x = event.delta.x;
            velocity.y = event.delta.y;
        }
    }
}

/// Rotate the player around himself/herself.
///
/// Move the mouse around the player to make him rotate.
pub fn rotate_player_write(
    player: Query<(Entity, &Transform), With<Player>>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mut events: EventWriter<PlayerRotateEvent>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window = query_window.single();

    // Checks that cursor is inside the window
    // The converts its coordinates to the world
    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok((entity, transform)) = player.get_single() {
            let player_position = transform.translation.truncate();
            let direction = cursor_position - player_position;
            let angle = direction.y.atan2(direction.x);

            events.send(PlayerRotateEvent {
                to: Quat::from_rotation_z(angle),
                source: EventSource::Input,
                entity,
            });
        }
    }
}

pub fn rotate_player_read(
    mut player: Query<&mut Transform, With<Player>>,
    mut events: EventReader<PlayerRotateEvent>,
) {
    for event in events.read() {
        if let Ok(mut transform) = player.get_mut(event.entity) {
            transform.rotation = event.to;
        }
    }
}

/// Attacks with player weapon.
///
/// Use left mouse click to perform an attack with the player's weapon.
pub fn player_attack_write(
    mut player: Query<(Entity, &mut PlayerStats), With<Player>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut events: EventWriter<PlayerAttackEvent>,
) {
    let Ok((entity, mut stats)) = player.get_single_mut() else {
        return;
    };
    stats.attack.attack_speed.tick(time.delta());

    if !mouse.pressed(MouseButton::Left) {
        return;
    }
    if stats.attack.attack_speed.finished() {
        events.send(PlayerAttackEvent {
            entity,
            source: EventSource::Input,
        });
        stats.attack.attack_speed.reset();
    }
}

pub fn player_attack_read(
    mut commands: Commands,
    player: Query<(&Transform, &PlayerStats)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<PlayerAttackEvent>,
) {
    for event in events.read() {
        if let Ok((transform, stats)) = player.get(event.entity) {
            let attack = &stats.attack;

            // Projectile size
            let width = attack.size.x;
            let height = attack.size.y;

            // Projectile transform
            let position =
                transform.translation + transform.rotation * Vec3::X * (PLAYER_RADIUS + 10.0);
            let rotation = transform.rotation;

            // Projectile movement
            let direction = position - transform.translation;
            let speed = attack.speed;
            let velocity = direction * speed;

            commands.spawn((
                AttackProjectile::new(transform.translation.truncate(), attack.range, stats.damage),
                ColorMesh2dBundle {
                    mesh: meshes.add(Rectangle::new(height, width)).into(),
                    material: materials.add(Color::linear_rgb(0.8, 0.6, 0.8)),
                    transform: Transform::from_translation(position).with_rotation(rotation),
                    ..default()
                },
                RigidBody::Dynamic,
                LinearVelocity(velocity.truncate()),
                Collider::rectangle(height, width),
            ));
        }
    }
}

pub fn despawn_out_of_range_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &Transform, &mut AttackProjectile)>,
) {
    for (entity, transform, attack_projectile) in projectiles.iter_mut() {
        let traveled_distance = transform
            .translation
            .truncate()
            .distance(attack_projectile.initial_position);

        if traveled_distance > attack_projectile.range {
            commands.entity(entity).despawn();
            debug!("Projectile out of range {entity:?}");
        }
    }
}

pub fn despawn_collided_projectiles(
    mut commands: Commands,
    projectiles: Query<(Entity, &CollidingEntities, &AttackProjectile)>,
    mut ennemy_query: Query<&mut EnnemyStats, With<Ennemy>>,
) {
    for (entity, colliding_entities, attack_projectile) in projectiles.iter() {
        for colliding_entity in colliding_entities.iter() {
            let Ok(mut ennemy_stats) = ennemy_query.get_mut(*colliding_entity) else {
                continue;
            };
            ennemy_stats.health -= attack_projectile.damage;

            if ennemy_stats.health <= 0.0 {
                commands.entity(*colliding_entity).despawn_recursive();
                debug!("Despawning enemy {colliding_entity:?}");
            }
        }

        if colliding_entities.len() > 0 {
            // despawn the projectile
            commands.entity(entity).despawn();
            debug!("Despawning projectile on collsion {entity:?}");
        }
    }
}

pub fn check_for_level_complete(
    query: Query<(), With<Ennemy>>,
    mut levels: ResMut<Levels>,
    mut current_level: ResMut<CurrentLevel>,
    mut game_state: ResMut<NextState<GameState>>,
    player_ghost_list: Res<PlayerGhostList>,
) {
    if query.iter().len() == 0 {
        let Some(level) = &current_level.0 else {
            warn!("No current level in check_for_level_complete");
            return;
        };
        info!("Completed level {} !!!!", level.id);
        levels.unlock_level(level.id + 1);
        levels.set_next_score(level.id, player_ghost_list.ghosts.len() + 1);
        current_level.0 = None;
        game_state.set(GameState::LevelSelection);
    }
}
