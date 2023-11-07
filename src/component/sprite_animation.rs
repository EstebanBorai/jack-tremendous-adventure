use bevy::prelude::Component;

/// Stores the time since the last frame.
#[derive(Component)]
pub struct FrameTime(pub f32);

/// Configures a sprite's animation by specifying how many sprites the
/// animation holds and how many time each spite lasts.
#[derive(Clone, Component)]
pub struct SpriteAnimation {
    pub len: usize,
    pub frame_time: f32,
}

impl SpriteAnimation {
    pub fn new(len: usize, fps: isize) -> Self {
        Self {
            len,
            frame_time: 1. / fps as f32,
        }
    }
}
