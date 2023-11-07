use bevy::prelude::Component;

pub const FALL_SPEED: f32 = 95.;

#[derive(Component, Debug)]
pub struct Jump(pub f32);
