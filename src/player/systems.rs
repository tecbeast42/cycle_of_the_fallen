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
) {
    commands
        .spawn((
            Player,
            PlayerStats::new(Class::Ranger),
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
    player: Query<(&Transform, &PlayerStats), With<Player>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if let Ok((transform, stats)) = player.get_single() {
        if mouse.just_pressed(MouseButton::Left) {
            let projectile_width = stats.attack.size.x;
            let projectile_height = stats.attack.size.y;

            commands.spawn((
                AttackProjectile,
                ColorMesh2dBundle {
                    mesh: meshes
                        .add(Rectangle::new(projectile_width, projectile_height))
                        .into(),
                    material: materials.add(Color::linear_rgb(0.8, 0.6, 0.8)),
                    transform: Transform::from_translation(
                        transform.translation + Vec3::new(PLAYER_RADIUS + 2.0, 0.0, 0.0),
                    )
                    .with_rotation(transform.rotation),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::rectangle(projectile_width, projectile_height),
            ));
        }
    }
}

/// Move projectiles around.
///
/// Projectiles move depending on their speed and range.
pub fn move_projectiles(player: Query<(), With<AttackProjectile>>) {}
