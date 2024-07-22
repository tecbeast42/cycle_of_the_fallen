use crate::level::history::*;
use avian2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Controllable;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Speed(pub f32);

#[derive(Event, Debug, Clone)]
pub struct CharacterMoveEvent {
    pub entity: Entity,
    pub delta: Vec2,
    pub source: EventSource,
}

impl SetEntity for CharacterMoveEvent {
    fn set_entity(&mut self, entity: Entity) {
        self.entity = entity;
    }
}

impl EventSourceMethods for CharacterMoveEvent {
    fn get_source(&self) -> EventSource {
        self.source
    }

    fn set_source(&mut self, source: EventSource) {
        self.source = source;
    }
}

impl EventRecordDebug for CharacterMoveEvent {
    fn get_debug_color(&self) -> Color {
        Color::srgba(0.0, 1.0, 0.0, 0.5)
    }
}

#[derive(Event, Debug)]
pub struct GameOverEvent;

pub fn spawn_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material: materials.add(Color::srgb(1.0, 0.5, 0.5)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Controllable,
        Speed(10.0),
    ));
}

pub fn character_move_input(
    mut move_event: EventWriter<CharacterMoveEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    controllables_query: Query<Entity, With<Controllable>>,
) {
    let mut delta = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        delta.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        delta.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        delta.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        delta.x += 1.0;
    }
    if delta.length() <= 0.0 {
        return;
    }

    delta = delta.normalize();

    for c in controllables_query.iter() {
        move_event.send(CharacterMoveEvent {
            entity: c,
            delta,
            source: EventSource::Input,
        });
    }
}

pub fn character_move_read(
    mut move_event: EventReader<CharacterMoveEvent>,
    mut targets: Query<(&mut Transform, &Speed)>,
    time: Res<Time>,
) {
    for e in move_event.read() {
        match targets.get_mut(e.entity) {
            Ok((mut transform, speed)) => {
                let delta = e.delta * speed.0 * time.delta_seconds();
                transform.translation.x += delta.x;
                transform.translation.y += delta.y;
            }
            Err(_) => {
                error!(
                    "Character not found (source: {:?}): {:?}",
                    e.source, e.entity
                );
            }
        }
    }
}

pub fn character_kill(
    mut commands: Commands,
    player_query: Query<(Entity, &Speed), With<Controllable>>,
    mut ghost_list: ResMut<PlayerGhostList>,
    mut game_over: EventWriter<GameOverEvent>,
) {
    for (e, speed) in player_query.iter() {
        commands.entity(e).despawn_recursive();

        ghost_list.ghosts.push(PlayerGhost {
            speed: speed.clone(),
            entity: None,
        });
    }

    game_over.send(GameOverEvent);
}
