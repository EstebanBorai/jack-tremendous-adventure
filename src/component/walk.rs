use bevy::prelude::Component;

#[derive(Debug, Component, PartialEq, Eq)]
pub enum Walk {
    Right,
    Left,
}
