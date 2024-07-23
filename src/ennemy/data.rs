use std::time::Duration;

use bevy::prelude::*;

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
}

/// Indicate that this [`Ennemy`] will always
/// attack the player no mather what
#[derive(Component)]
pub struct AlwaysAttack;
