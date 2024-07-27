use avian2d::prelude::*;
use bevy::log::tracing_subscriber::layer;

pub const PLAYER_CHARACTER_COLLISION_LAYER: u32 = 1 << 0;
pub const ENEMY_CHARACTER_COLLISION_LAYER: u32 = 1 << 1;
pub const PLAYER_PROJECTILE_COLLISION_LAYER: u32 = 1 << 2;
pub const ENEMY_PROJECTILE_COLLISION_LAYER: u32 = 1 << 3;
pub const WALL_COLLISION_LAYER: u32 = 1 << 4;

pub fn get_collision_layers(layer: u32) -> CollisionLayers {
    CollisionLayers::from_bits(
        layer,
        match layer {
            PLAYER_CHARACTER_COLLISION_LAYER => {
                ENEMY_PROJECTILE_COLLISION_LAYER
                    | ENEMY_CHARACTER_COLLISION_LAYER
                    | WALL_COLLISION_LAYER
            }
            ENEMY_CHARACTER_COLLISION_LAYER => {
                PLAYER_PROJECTILE_COLLISION_LAYER
                    | PLAYER_CHARACTER_COLLISION_LAYER
                    | WALL_COLLISION_LAYER
            }
            PLAYER_PROJECTILE_COLLISION_LAYER => {
                ENEMY_CHARACTER_COLLISION_LAYER
                    | ENEMY_PROJECTILE_COLLISION_LAYER
                    | WALL_COLLISION_LAYER
            }
            ENEMY_PROJECTILE_COLLISION_LAYER => {
                PLAYER_CHARACTER_COLLISION_LAYER
                    | PLAYER_PROJECTILE_COLLISION_LAYER
                    | WALL_COLLISION_LAYER
            }
            WALL_COLLISION_LAYER => {
                PLAYER_CHARACTER_COLLISION_LAYER
                    | ENEMY_CHARACTER_COLLISION_LAYER
                    | PLAYER_PROJECTILE_COLLISION_LAYER
                    | ENEMY_PROJECTILE_COLLISION_LAYER
            }
            _ => LayerMask::ALL.0,
        },
    )
}
