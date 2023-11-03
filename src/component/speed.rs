use bevy::prelude::Component;

/// Determines the speed of an entity.
#[derive(Debug, Component)]
pub struct Speed(pub f32);
