use bevy::prelude::{Component, Query, Res};
use bevy::sprite::TextureAtlasSprite;
use bevy::time::Time;

/// Stores the time since the last frame.
#[derive(Component)]
pub struct FrameTime(pub f32);

/// Configures a sprite's animatio by specifying how many sprites the
/// animation holds and how many time each spite lasts.
#[derive(Clone, Component)]
pub struct SpriteAnimation {
    pub len: usize,
    pub frame_time: f32,
}

pub fn animate_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        // Time since the last frame
        frame_time.0 += time.delta_seconds();

        if frame_time.0 >= animation.frame_time {
            // Calculates how many frames passed by calculating the time and
            // each animation time. Then decimals are forgotten by casting to
            // usize.
            let frames = (frame_time.0 / animation.frame_time) as usize;

            sprite.index += frames;

            if sprite.index >= animation.len {
                sprite.index %= animation.len;
            }

            frame_time.0 -= animation.frame_time * frames as f32;
        }
    }
}
