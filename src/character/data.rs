use bevy::prelude::*;

use crate::Class;

#[derive(Component)]
pub struct CharacterSelectionButton(pub Class);

#[derive(Resource)]
pub struct SelectedCharacter(pub Class);

impl Default for SelectedCharacter {
    fn default() -> Self {
        SelectedCharacter(Class::Knight)
    }
}

impl SelectedCharacter {
    pub fn set(&mut self, class: Class) {
        self.0 = class;
    }
}
