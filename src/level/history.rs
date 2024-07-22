use crate::character::*;
use avian2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Resource)]
pub struct LevelHistory<T: Event> {
    pub events: Vec<EventRecord<T>>,
}

impl<E: Event> Default for LevelHistory<E> {
    fn default() -> Self {
        Self { events: vec![] }
    }
}

#[derive(Resource, Default)]
pub struct LevelStartTime(pub f64);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct GhostIdentifier(pub usize);

#[derive(Default)]
pub struct PlayerGhost {
    pub speed: Speed,
    pub entity: Option<Entity>,
}

#[derive(Event, Debug, Clone)]
pub struct EventRecord<E: Event> {
    pub ghost: GhostIdentifier,
    pub timestamp: f64,
    pub event: E,
}

#[derive(Resource, Default)]
pub struct PlayerGhostList {
    pub ghosts: Vec<PlayerGhost>,
}

impl PlayerGhostList {
    pub fn get_ghost(&self, identifier: GhostIdentifier) -> Option<&PlayerGhost> {
        self.ghosts.get(identifier.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventSource {
    Replay,
    Input,
}

pub trait SetEntity {
    fn set_entity(&mut self, entity: Entity);
}

pub trait EventSourceMethods {
    fn get_source(&self) -> EventSource;
    fn set_source(&mut self, source: EventSource);
}

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

pub fn reset_game_history_start(mut history: ResMut<LevelStartTime>, time: Res<Time>) {
    history.0 = time.elapsed().as_secs_f64();
    info!("Reset game history start time to {}", history.0);
}

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

pub fn spawn_ghosts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ghost_list: ResMut<PlayerGhostList>,
) {
    let ghost_mesh = meshes.add(Circle { radius: 10.0 });

    for (i, g) in ghost_list.ghosts.iter_mut().enumerate() {
        if let Some(e) = g.entity {
            warn!("Ghost {:?} already exists", e);
            commands.entity(e).despawn_recursive();
        }
        let entity = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(ghost_mesh.clone()),
                    material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                GhostIdentifier(i),
                g.speed.clone(),
            ))
            .id();

        g.entity = Some(entity);
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

pub trait EventRecordDebug {
    fn get_debug_color(&self) -> Color;
}

pub fn debug_history<E: Event + EventRecordDebug>(
    mut commands: Commands,
    history: Res<LevelHistory<E>>,
    time: Res<Time>,
    start_time: Res<LevelStartTime>,
    mut container_id: Local<Option<Entity>>,
) {
    if let Some(container) = container_id.as_ref() {
        commands.entity(*container).despawn_recursive();
    }
    let delta = time.delta_seconds_f64();
    let start = time.elapsed().as_secs_f64() - delta - start_time.0;
    let scale = 100.0;

    let height = Val::Px(50.0);
    let container = commands
        .spawn(NodeBundle {
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
        })
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
            let color = record.event.get_debug_color();

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
