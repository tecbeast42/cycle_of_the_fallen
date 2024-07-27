use crate::player::prelude::*;
use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct SavePlayerGhostEvent {
    pub class: Class,
}
