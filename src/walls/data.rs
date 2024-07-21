use bevy::prelude::*;

pub const WALL_WIDTH: f32 = 20.0;

/// Marker component of the walls.
///
/// This component allow to identify the walls during Bevy queries.
#[derive(Component, Debug)]
pub struct Wall;
