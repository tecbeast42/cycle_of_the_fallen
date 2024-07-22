use crate::player::prelude::PlayerStats;
use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelHistory<T: Event> {
    pub events: Vec<EventRecord<T>>,
}

pub struct PlayerGhost {
    pub entity: Option<Entity>,
    pub stats: PlayerStats,
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

impl<E: Event> Default for LevelHistory<E> {
    fn default() -> Self {
        Self { events: vec![] }
    }
}

#[derive(Resource, Default)]
pub struct LevelStartTime(pub f64);

#[derive(Debug, Clone)]
pub struct EventRecord<E: Event> {
    pub ghost: GhostIdentifier,
    pub timestamp: f64,
    pub event: E,
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

pub trait EventRecordDebug {
    fn get_debug_color(&self, identifier: GhostIdentifier) -> Color;
}

/// Ghost index identifier
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct GhostIdentifier(pub usize);