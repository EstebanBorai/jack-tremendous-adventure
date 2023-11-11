use bevy::prelude::Component;

#[derive(Debug, PartialEq, Eq)]
pub enum RunDirection {
    Right,
    Left,
}

#[derive(Debug, Component, PartialEq)]
pub struct Run {
    pub speed: f32,
    pub direction: RunDirection,
}
