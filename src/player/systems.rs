use crate::character::prelude::SelectedCharacter;

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
            PlayerStats::new(selected_character.0.clone()),
            LastAttack(None),
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

/// Moves the player around.
///
/// Use ZQSD (or WASD) to move the player around.
pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut LinearVelocity, With<Player>>,
) {
    let delta_time = time.delta_seconds();

    let mut direction = Vec2::ZERO;

    for mut linear_velocity in &mut player {
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

        linear_velocity.x = direction.x;
        linear_velocity.y = direction.y;
    }
}

/// Rotate the player around himself/herself.
///
/// Move the mouse around the player to make him rotate.
pub fn rotate_player(
    mut player: Query<&mut Transform, With<Player>>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
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
        if let Ok(mut transform) = player.get_single_mut() {
            let player_position = transform.translation.truncate();
            let direction = cursor_position - player_position;
            let angle = direction.y.atan2(direction.x);

            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

/// Attacks with player weapon.
///
/// Use left mouse click to perform an attack with the player's weapon.
pub fn attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player: Query<(&Transform, &PlayerStats, &mut LastAttack), With<Player>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    if let Ok((transform, stats, mut last_attack)) = player.get_single_mut() {
        let attack = &stats.attack;

        if mouse.pressed(MouseButton::Left) {
            if let Some(timer) = &mut last_attack.0 {
                timer.tick(time.delta());
            }

            let first_shot = last_attack.0.is_none();

            let delayed_enough = last_attack.0.clone().is_some_and(|timer| timer.finished());

            if first_shot || delayed_enough {
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
                    AttackProjectile::new(transform.translation.truncate(), attack.range),
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

                last_attack.0 = Some(Timer::from_seconds(attack.attack_speed, TimerMode::Once));
            }
        }
    }
}

pub fn despawn_collided_projectiles(
    mut commands: Commands,
    projectiles: Query<(Entity, &CollidingEntities), With<AttackProjectile>>,
) {
    for (entity, colliding_entities) in projectiles.iter() {
        if colliding_entities.len() > 0 {
            commands.entity(entity).despawn();
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
        }
    }
}
