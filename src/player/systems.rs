use crate::game::{CurrentLevel, GameState, Levels};
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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn((
            StateScoped(GameState::Play),
            Player,
            PlayerBundle::new(
                selected_character.0,
                &asset_server,
                &mut texture_atlas_layouts,
            ),
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
/// Use WASD to move the player around.
pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut LinearVelocity, &mut Animation, &mut TextureAtlas), With<Player>>,
) {
    let delta_time = time.delta_seconds();

    let mut direction = Vec2::ZERO;

    for (mut velocity, mut animation, mut atlas) in &mut player {
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
        animation.travelled += direction.length();

        if animation.travelled >= 800.0 {
            atlas.index = if atlas.index == animation.indices.1 {
                animation.indices.0
            } else {
                atlas.index + 1
            };
            animation.travelled -= 800.0
        }

        velocity.x = direction.x;
        velocity.y = direction.y;
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
pub fn player_attack(
    mut player: Query<(&Transform, &mut PlayerStats), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let Ok((transform, mut stats)) = player.get_single_mut() else {
        return;
    };
    stats.attack.attack_speed.tick(time.delta());

    if !mouse.pressed(MouseButton::Left) {
        return;
    }
    if stats.attack.attack_speed.finished() {
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
            StateScoped(GameState::Play),
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
            CollisionLayers::from_bits(0b1000, 0b0001),
        ));
        stats.attack.attack_speed.reset();
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

pub fn handle_projectile_colissions(
    mut commands: Commands,
    projectiles: Query<(Entity, &CollidingEntities), With<AttackProjectile>>,
    ennemy_query: Query<(), With<Ennemy>>,
    player_query: Query<&Class, With<Player>>,
) {
    for (entity, colliding_entities) in projectiles.iter() {
        for colliding_entity in colliding_entities.iter() {
            if ennemy_query.get(*colliding_entity).is_ok() {
                debug!(
                    "Despawning projectile and enemy on collsion {entity:?} {colliding_entity:?}"
                );
                commands.entity(*colliding_entity).despawn_recursive();
                commands.entity(entity).despawn_recursive();
            }
            if let Ok(class) = player_query.get(*colliding_entity) {
                debug!(
                    "Despawning projectile and player on collsion {entity:?} {colliding_entity:?}"
                );
                commands.entity(*colliding_entity).despawn_recursive();
                commands.entity(entity).despawn_recursive();
            };
        }
    }
}

fn player_died(mut removals: RemovedComponents<Player>, mut commands: Commands) {
    for entity in removals.read() {
        // create a new ghost
        // start replay from beginnig
    }
}

/// TODO reenable score recodring
pub fn check_for_level_complete(
    query: Query<(), With<Ennemy>>,
    mut levels: ResMut<Levels>,
    mut current_level: ResMut<CurrentLevel>,
    mut game_state: ResMut<NextState<GameState>>,
    // player_ghost_list: Res<PlayerGhostList>,
) {
    if query.iter().len() == 0 {
        let Some(level) = &current_level.0 else {
            warn!("No current level in check_for_level_complete");
            return;
        };

        info!("Completed level {} !!!!", level.id);

        if levels.0.len() == level.id + 1 {
            levels.unlock_level(level.id + 1);
        }

        // levels.set_next_score(level.id, player_ghost_list.ghosts.len() + 1);
        current_level.0 = None;
        game_state.set(GameState::LevelSelection);
    }
}
