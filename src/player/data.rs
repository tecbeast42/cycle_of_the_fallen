use super::prelude::*;
use avian2d::prelude::*;
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
#[derive(Component, Clone, Debug, Copy, PartialEq, Eq)]
pub enum Class {
    Knight,
    Ranger,
    Wizard,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    Alive,
    Ghost,
}

// Marker component indicating that the entity can be targeted
#[derive(Component, Debug, Clone, Copy)]
pub struct Targetable;

/// The different weapons of the game.
///
/// - The sword does melee damage.
/// - The bow inflicts damage from far range but has low damage.
/// - The staff inflicts damage from medium range but with high damage and slow travel time.
#[derive(Clone, Debug)]
pub enum Weapon {
    Sword,
    Bow,
    Staff,
}

/// The statics of a player.
///
/// This statistics are those of a player (the current one or a ghost).
/// They are defined by the class of that given player
#[derive(Component, Debug, Clone)]
pub struct PlayerStats {
    pub attack: Attack,
}

impl PlayerStats {
    pub fn new(class: Class) -> Self {
        match class {
            Class::Knight => Self {
                attack: Attack::new(Weapon::Sword),
            },
            Class::Ranger => Self {
                attack: Attack::new(Weapon::Bow),
            },
            Class::Wizard => Self {
                attack: Attack::new(Weapon::Staff),
            },
        }
    }
}

/// The statics of the attacks.
///
/// This statistics are those of the player attacks (the current one or a ghost).
/// They are defined by the class of that given player
#[derive(Component, Debug, Clone)]
pub struct Attack {
    pub size: Vec2,
    pub speed: f32,
    pub range: f32,
    pub attack_speed: Timer,
}

impl Attack {
    pub fn new(weapon: Weapon) -> Self {
        match weapon {
            Weapon::Sword => Self {
                size: Vec2::new(12.0, 12.0),
                speed: 0.0,
                range: 10.0,
                attack_speed: Timer::from_seconds(0.3, TimerMode::Once),
            },
            Weapon::Bow => Self {
                size: Vec2::new(3.0, 8.0),
                speed: 30.0,
                range: 600.0,
                attack_speed: Timer::from_seconds(0.1, TimerMode::Once),
            },
            Weapon::Staff => Self {
                size: Vec2::new(8.0, 8.0),
                speed: 15.0,
                range: 400.0,
                attack_speed: Timer::from_seconds(0.3, TimerMode::Once),
            },
        }
    }
}

/// Marker component of the weapon projectiles/attacks.
///
/// This component allow to identify the attacks during Bevy queries.
#[derive(Component, Debug)]
pub struct AttackProjectile {
    pub initial_position: Vec2,
    pub range: f32,
}

impl AttackProjectile {
    pub fn new(initial_position: Vec2, range: f32) -> Self {
        AttackProjectile {
            initial_position,
            range,
        }
    }
}

/// Component to handle the sprites
#[derive(Component, Debug)]
pub struct Animation {
    pub indices: (usize, usize),
    pub travelled: f32,
}

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub name: Name,
    pub class: Class,
    pub player_stats: PlayerStats,
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animation: Animation,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub player_type: PlayerType,
    pub team: Team,
    pub targetable: Targetable,
    pub collision_layers: CollisionLayers,
}

impl PlayerBundle {
    pub fn new(
        player_type: PlayerType,
        class: Class,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let texture = match class {
            Class::Knight => asset_server.load("knight.png"),
            Class::Ranger => asset_server.load("ranger.png"),
            Class::Wizard => asset_server.load("wizard.png"),
        };
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(256), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let collision_layers = get_collision_layers(PLAYER_CHARACTER_COLLISION_LAYER);

        PlayerBundle {
            name: Name::new("Player"),
            player_stats: PlayerStats::new(class),
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform::from_xyz(-400.0, 0.0, 0.0)
                    .with_scale(Vec3::new(0.3, 0.3, 0.3)),
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            animation: Animation {
                indices: (0, 3),
                travelled: 0.0,
            },
            class,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(PLAYER_RADIUS),
            player_type,
            team: Team::Player,
            targetable: Targetable,
            collision_layers,
        }
    }
}

// PlayerStats::new(class),
// SpriteBundle {
//     texture,
//     transform: Transform::from_xyz(-400.0, 0.0, 0.0)
//         .with_scale(Vec3::new(0.3, 0.3, 0.3)),
//     ..default()
// },
// TextureAtlas {
//     layout: texture_atlas_layout,
//     index: 0,
// },
// Animation {
//     indices: (0, 3),
//     travelled: 0.0,
// },
// RigidBody::Dynamic,
// Collider::circle(PLAYER_RADIUS),
