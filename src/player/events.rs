use crate::level_history::prelude::*;
use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct PlayerMoveEvent {
    pub entity: Entity,
    pub source: EventSource,
    pub delta: Vec2,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerRotateEvent {
    pub entity: Entity,
    pub source: EventSource,
    pub to: Quat,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerAttackEvent {
    pub entity: Entity,
    pub source: EventSource,
}

impl SetEntity for PlayerMoveEvent {
    fn set_entity(&mut self, entity: Entity) {
        self.entity = entity;
    }
}
impl SetEntity for PlayerRotateEvent {
    fn set_entity(&mut self, entity: Entity) {
        self.entity = entity;
    }
}
impl SetEntity for PlayerAttackEvent {
    fn set_entity(&mut self, entity: Entity) {
        self.entity = entity;
    }
}

impl EventSourceMethods for PlayerMoveEvent {
    fn set_source(&mut self, source: EventSource) {
        self.source = source;
    }
    fn get_source(&self) -> EventSource {
        self.source
    }
}

impl EventSourceMethods for PlayerRotateEvent {
    fn set_source(&mut self, source: EventSource) {
        self.source = source;
    }
    fn get_source(&self) -> EventSource {
        self.source
    }
}

impl EventSourceMethods for PlayerAttackEvent {
    fn set_source(&mut self, source: EventSource) {
        self.source = source;
    }
    fn get_source(&self) -> EventSource {
        self.source
    }
}

impl EventRecordDebug for PlayerMoveEvent {
    fn get_debug_color(&self, _: GhostIdentifier) -> Color {
        Color::srgba(0.0, 0.0, 1.0, 0.2)
    }
}

impl EventRecordDebug for PlayerRotateEvent {
    fn get_debug_color(&self, _: GhostIdentifier) -> Color {
        Color::srgba(0.0, 1.0, 0.0, 0.2)
    }
}

impl EventRecordDebug for PlayerAttackEvent {
    fn get_debug_color(&self, _: GhostIdentifier) -> Color {
        Color::srgba(1.0, 0.0, 1.0, 0.5)
    }
}