use std::time::Duration;

use crate::player::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub enum AttackSpeedType {
    Regular,
}

/// Marker component of the enemies.
///
/// This component allow to identify the enemies during Bevy queries.
#[derive(Component, Debug)]
pub struct Ennemy;

/// The different enemies of the game.
///
/// - The knight have lot of health and does damage with melee attacks.
/// - The ranger shoots from long range but with low damage.
/// - The wizard inflicts high damages at medium range but is very weak.
#[derive(Component, Debug, Clone, Copy)]
pub enum EnnemyKind {
    Dummy,
    Turret,
}

impl EnnemyKind {
    pub fn radius(&self) -> f32 {
        match self {
            EnnemyKind::Dummy => 15.0,
            EnnemyKind::Turret => 15.0,
        }
    }
}

#[derive(Component)]
pub struct AttackSpeed(Timer);

impl AttackSpeed {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
    pub fn tick(&mut self, duration: Duration) {
        self.0.tick(duration);
    }
    pub fn reset(&mut self) {
        self.0.reset();
    }

    pub fn finished(&self) -> bool {
        self.0.finished()
    }
    pub fn from_type(attack_type: AttackSpeedType) -> Self {
        let time = match attack_type {
            AttackSpeedType::Regular => 2.0,
        };

        Self::new(Timer::from_seconds(time, TimerMode::Once))
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Ennemy,
    pub kind: EnnemyKind,
    pub targetable: Targetable,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
}

impl EnemyBundle {
    pub fn new(kind: EnnemyKind, radius: f32) -> Self {
        Self {
            enemy: Ennemy,
            kind,
            targetable: Targetable,
            rigid_body: RigidBody::Static,
            collider: Collider::circle(radius),
            collision_layers: CollisionLayers::from_bits(0b0001, 0b1000),
        }
    }
}
