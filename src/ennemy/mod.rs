mod data;
mod systems;

use bevy::prelude::*;
use systems::*;

pub mod prelude {
    pub use super::data::*;
    pub use super::EnnemyPlugin;
}

pub struct EnnemyPlugin;

impl Plugin for EnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ennemies);
    }
}
