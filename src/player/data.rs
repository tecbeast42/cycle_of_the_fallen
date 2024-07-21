use bevy::prelude::*;

pub const PLAYER_RADIUS: f32 = 15.0;

/// Marker component of the player.
///
/// This component allow to identify the playable character during Bevy queries.
#[derive(Component, Debug)]
pub struct Player;

/// The different classes of the game.
///
/// - The knight have lot of health and does damage with melee attacks.
/// - The ranger shoots from long range but with low damage.
/// - The wizard inflicts hight damages at medium range but is very weak.
pub enum Class {
    Knight,
    Ranger,
    Wizard,
}

/// The different weapons of the game.
///
/// - The sword does melee damage.
/// - The bow inflicts damage from far range but has low damage.
/// - The staff inflicts damage from medium range but with high damage and slow travel time.
pub enum Weapon {
    Sword,
    Bow,
    Staff,
}

/// The statics of a player.
///
/// This statistics are those of a player (the current one or a ghost).
/// They are defined by the class of that given player
#[derive(Component, Debug)]
pub struct PlayerStats {
    pub health: f32,
    pub damage: f32,
    pub attack: Attack,
}

impl PlayerStats {
    pub fn new(class: Class) -> Self {
        match class {
            Class::Knight => Self {
                health: 100.0,
                damage: 10.0,
                attack: Attack::new(Weapon::Sword),
            },
            Class::Ranger => Self {
                health: 100.0,
                damage: 10.0,
                attack: Attack::new(Weapon::Bow),
            },
            Class::Wizard => Self {
                health: 100.0,
                damage: 10.0,
                attack: Attack::new(Weapon::Staff),
            },
        }
    }
}

/// The statics of the attacks.
///
/// This statistics are those of the player attacks (the current one or a ghost).
/// They are defined by the class of that given player
#[derive(Component, Debug)]
pub struct Attack {
    pub ranged: bool,
    pub size: Vec2,
    pub speed: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
}

impl Attack {
    pub fn new(weapon: Weapon) -> Self {
        match weapon {
            Weapon::Sword => Self {
                ranged: false,
                size: Vec2::new(12.0, 12.0),
                speed: 0.0,
                attack_range: 5.0,
                attack_speed: 30.0,
            },
            Weapon::Bow => Self {
                ranged: true,
                size: Vec2::new(3.0, 8.0),
                speed: 30.0,
                attack_range: 5.0,
                attack_speed: 30.0,
            },
            Weapon::Staff => Self {
                ranged: true,
                size: Vec2::new(12.0, 12.0),
                speed: 0.0,
                attack_range: 5.0,
                attack_speed: 30.0,
            },
        }
    }
}

/// Marker component of the weapon projectiles/attacks.
///
/// This component allow to identify the attacks during Bevy queries.
#[derive(Component, Debug)]
pub struct AttackProjectile;
