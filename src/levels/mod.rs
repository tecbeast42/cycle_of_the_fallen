mod data;
mod systems;

use bevy::prelude::*;
use systems::*;

pub mod prelude {
    pub use super::data::*;
    pub use super::LevelsPlugin;
}

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
    }
}
