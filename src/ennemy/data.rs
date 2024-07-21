use bevy::prelude::*;

pub const TURRET_RADIUS: f32 = 15.0;

/// Marker component of the ennemies.
///
/// This component allow to identify the ennemies during Bevy queries.
#[derive(Component, Debug)]
pub struct Ennemy;

/// The different enemies of the game.
///
/// - The knight have lot of health and does damage with melee attacks.
/// - The ranger shoots from long range but with low damage.
/// - The wizard inflicts hight damages at medium range but is very weak.
pub enum EnnemyType {
    Dummy,
    Turret,
}

/// The statics of an ennemy.
///
/// This statistics are those of an ennemy.
/// They are defined by the class of that given ennemy
#[derive(Component, Debug)]
pub struct EnnemyStats {
    pub health: f32,
    pub damage: f32,
    pub projectile_size: Vec2,
    pub projectile_speed: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
}

impl EnnemyStats {
    pub fn new(enemy_type: EnnemyType) -> Self {
        match enemy_type {
            EnnemyType::Dummy => Self {
                health: 100.0,
                damage: 0.0,
                projectile_size: Vec2::new(10.0, 10.0),
                projectile_speed: 1.0,
                attack_range: 1.0,
                attack_speed: 0.5,
            },
            EnnemyType::Turret => Self {
                health: 200.0,
                damage: 10.0,
                projectile_size: Vec2::new(10.0, 10.0),
                projectile_speed: 15.0,
                attack_range: 50.0,
                attack_speed: 30.0,
            },
        }
    }
}
