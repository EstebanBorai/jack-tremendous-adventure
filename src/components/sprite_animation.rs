use bevy::{
    animation,
    prelude::{Component, Query, Res},
    sprite::TextureAtlasSprite,
    time::Time,
};

#[derive(Component)]
pub struct FrameTime(f32);

#[derive(Component)]
pub struct SpriteAnimation {
    len: usize,
    frame_time: f32,
}

pub fn animate_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        todo!()
    }
}
