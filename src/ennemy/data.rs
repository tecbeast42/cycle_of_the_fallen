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

/// Indicate that this [`Ennemy`] will always
/// attack the player no mather what
#[derive(Component)]
pub struct AlwaysAttack;

#[derive(Bundle)]
pub struct EnemyBundle {
    name: Name,
    pub enemy: Ennemy,
    pub kind: EnnemyKind,
    pub team: Team,
    pub targetable: Targetable,
    pub mesh: ColorMesh2dBundle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
}

impl EnemyBundle {
    pub fn new(
        kind: EnnemyKind,
        radius: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            name: Name::new("Enemy"),
            enemy: Ennemy,
            kind,
            team: Team::Enemy,
            targetable: Targetable,
            mesh: ColorMesh2dBundle {
                mesh: meshes.add(Circle::new(radius)).into(),
                material: materials.add(Color::linear_rgb(0.6, 0.2, 0.1)),
                transform: Transform::from_xyz(300.0, 100.0, 0.0),
                ..default()
            },
            rigid_body: RigidBody::Static,
            collider: Collider::circle(radius),
            collision_layers: get_collision_layers(ENEMY_CHARACTER_COLLISION_LAYER),
        }
    }
}
