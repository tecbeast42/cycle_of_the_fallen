mod data;
mod systems;

use bevy::prelude::*;
use systems::*;

pub mod prelude {
    pub use super::data::*;
    pub use super::WallPlugin;
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}
