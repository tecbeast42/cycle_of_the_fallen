use bevy::{app::FixedMain, prelude::*, utils::HashMap};

use crate::{game::GameState, Class, Ghost, Player, PlayerBundle};

/// a component to indicate which entry on the [`ReplayStorage`]
/// is replayed through this ghost
#[derive(Component)]
struct GhostFor(Entity);

#[derive(Resource, Default)]
struct ReplayStorage {
    transform: HashMap<(Entity, Class), Vec<Transform>>,
    offset: usize,
}

impl ReplayStorage {
    fn clear(&mut self) {
        self.transform.clear();
        self.offset = 0;
    }
}

pub struct ReplayPlugin;
impl Plugin for ReplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedMain, (record_player, replay))
            .add_systems(OnEnter(GameState::LevelSelection), reset_storage)
            .add_systems(OnEnter(GameState::Play), reset_replay)
            .init_resource::<ReplayStorage>();
        // record player movement (translation)
        // record player aiming (rotation)
        // record player attacking (attack)
    }
}

fn record_player(
    query: Query<(Entity, &Transform, &Class), With<Player>>,
    mut replay_storage: ResMut<ReplayStorage>,
) {
    for (entity, transform, class) in query.iter() {
        replay_storage
            .transform
            .get_mut(&(entity, *class))
            .unwrap_or(&mut Vec::new())
            .push(*transform);
    }
}

fn reset_replay(
    mut replay_storage: ResMut<ReplayStorage>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    replay_storage.offset = 0;
    for ((entity, class), _) in replay_storage.transform.iter() {
        commands.spawn((
            Ghost,
            GhostFor(*entity),
            StateScoped(GameState::Play),
            PlayerBundle::new(*class, &asset_server, &mut texture_atlas_layouts),
        ));
    }
}

fn replay(
    mut replay_storage: ResMut<ReplayStorage>,
    mut query: Query<(&mut Transform, &GhostFor), With<Ghost>>,
) {
    for ((entity, _), vec_transform) in replay_storage.transform.iter() {
        // find the correct ghost
        for (mut ghost_transform, ghost_for) in query.iter_mut() {
            if *entity != ghost_for.0 {
                continue;
            }
            let Some(transform) = vec_transform.get(replay_storage.offset) else {
                continue;
            };

            *ghost_transform = *transform;
        }
    }

    replay_storage.offset += 1;
}

fn reset_storage(mut replay_storage: ResMut<ReplayStorage>) {
    replay_storage.clear();
}
