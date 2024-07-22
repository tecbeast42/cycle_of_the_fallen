use bevy::prelude::*;

use crate::game::Level;

/// holds temporary if the button represents
/// an unlocked level and which level it is
#[derive(Component, Default)]
pub struct LevelSelectionButton {
    pub level: usize,
    pub unlocked: bool,
}

impl From<&Level> for LevelSelectionButton {
    fn from(value: &Level) -> Self {
        LevelSelectionButton {
            level: value.id,
            unlocked: value.unlocked,
        }
    }
}
