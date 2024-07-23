use super::prelude::*;
use crate::game::GameState;
use crate::player::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

/// Records events
///
/// Events have a timestamp and a ghost identifier
/// The ghost identifier is used to identify the ghost
/// that caused the event
pub fn record_event<E: Event + EventSourceMethods + Clone + std::fmt::Debug>(
    mut history: ResMut<LevelHistory<E>>,
    ghost_list: Res<PlayerGhostList>,
    start_time: Res<LevelStartTime>,
    time: Res<Time>,
    mut event: EventReader<E>,
) {
    for e in event.read() {
        if e.get_source() == EventSource::Replay {
            continue;
        }
        let timestamp = time.elapsed().as_secs_f64() - start_time.0;
        let ghost = GhostIdentifier(ghost_list.ghosts.len());
        let mut recorded_event = e.clone();
        recorded_event.set_source(EventSource::Replay);
        history.events.push(EventRecord {
            ghost,
            timestamp,
            event: recorded_event,
        });
    }

    if !event.is_empty() {
        info!("Recorded events: {:?}", history.events.len())
    }
}

/// Replays events stored in the history
///
/// Sends events that were recorded in a time between now and the previous frame
pub fn replay_event<E: Event + Clone + std::fmt::Debug + SetEntity>(
    history: ResMut<LevelHistory<E>>,
    start_time: Res<LevelStartTime>,
    mut event_writer: EventWriter<E>,
    ghost_list: Res<PlayerGhostList>,
    time: Res<Time>,
) {
    for record in history.events.iter() {
        let timestamp = record.timestamp;
        let delta = time.delta_seconds_f64();
        let start = time.elapsed().as_secs_f64() - delta - start_time.0;
        let end = start + delta;

        let is_between = timestamp > start && timestamp <= end;

        if is_between {
            if let Some(entity) = ghost_list.get_ghost(record.ghost).and_then(|g| g.entity) {
                let mut event = record.event.clone();
                event.set_entity(entity);
                event_writer.send(event);
            }
        }
    }
}

/// Stores the start time of the level
///
/// This is used by the replay system to correctly interpret timestamps
pub fn reset_level_start_time(mut history: ResMut<LevelStartTime>, time: Res<Time>) {
    history.0 = time.elapsed().as_secs_f64();
    info!("Reset game history start time to {}", history.0);
}

pub fn debug_history<E: Event + EventRecordDebug>(
    mut commands: Commands,
    history: Res<LevelHistory<E>>,
    time: Res<Time>,
    start_time: Res<LevelStartTime>,
    mut container_id: Local<Option<Entity>>,
) {
    if let Some(entity_commands) = container_id.as_ref().and_then(|e| commands.get_entity(*e)) {
        entity_commands.despawn_recursive();
    }
    let delta = time.delta_seconds_f64();
    let start = time.elapsed().as_secs_f64() - delta - start_time.0;
    let scale = ((start / 100.0).min(1.0).ceil() * 100.0) as f32;

    let height = Val::Px(50.0);
    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height,
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
            StateScoped(GameState::Play),
        ))
        .id();

    let current_time_tracker = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(delta as f32 * scale),
                height,
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(start as f32 * scale),
                ..default()
            },
            background_color: Color::srgba(1.0, 1.0, 0.0, 0.5).into(),
            ..default()
        })
        .id();

    let mut records = history
        .events
        .iter()
        .map(|record| {
            let timestamp = record.timestamp;
            let color = record.event.get_debug_color(record.ghost);

            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(1.0),
                        height,
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        left: Val::Px(timestamp as f32 * scale),
                        ..default()
                    },
                    background_color: color.into(),
                    ..default()
                })
                .id()
        })
        .collect::<Vec<_>>();

    records.push(current_time_tracker);
    commands.entity(container).push_children(&records);
    *container_id = Some(container);
}

/// Clears the level history
pub fn clear_history<T: Event>(mut history: ResMut<LevelHistory<T>>) {
    history.events.clear();
}

/// Spawn previous recordings of the player
/// They will perform all actions that the player did
pub fn spawn_ghosts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ghost_list: ResMut<PlayerGhostList>,
) {
    let mut count = 0;
    for g in ghost_list.ghosts.iter_mut() {
        count += 1;
        if let Some(e) = g.entity {
            warn!("Ghost {:?} already exists", e);
            commands.entity(e).despawn_recursive();
        }
        let entity = commands
            .spawn((
                Ghost,
                g.stats.clone(),
                ColorMesh2dBundle {
                    mesh: meshes.add(Circle::new(PLAYER_RADIUS)).into(),
                    material: materials.add(Color::linear_rgb(0.0, 0.0, 1.0)),
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
            })
            .id();

        g.entity = Some(entity);
    }
    if count > 0 {
        info!("Spawned {} ghosts", count);
    } else {
        info!("No ghost to spawn");
    }
}

pub fn ghost_despawn(mut commands: Commands, mut ghost_list: ResMut<PlayerGhostList>) {
    for g in ghost_list.ghosts.iter_mut() {
        if let Some(e) = g.entity {
            commands.entity(e).despawn_recursive();
            g.entity = None;
        }
    }
}

pub fn save_player_ghost(
    mut ghost_list: ResMut<PlayerGhostList>,
    query_player: Query<&PlayerStats, With<Player>>,
) {
    if let Ok(player_stats) = query_player.get_single() {
        ghost_list.ghosts.push(PlayerGhost {
            stats: player_stats.clone(),
            entity: None,
        });
    } else {
        error!("No player to save");
    }
}
