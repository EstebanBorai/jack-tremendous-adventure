use bevy::prelude::{AssetServer, Assets, Commands, Component, Res, ResMut, Vec2};
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};

use crate::resources::image::MASK_DUDE_IDLE_32X32;

use super::sprite_animation::{FrameTime, SpriteAnimation};

#[derive(Debug, Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load(MASK_DUDE_IDLE_32X32),
        Vec2::splat(32.),
        11,
        1,
        None,
        None,
    );

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        },
        Player,
        SpriteAnimation {
            len: 11,
            frame_time: 1. / 20.,
        },
        FrameTime(0.),
    ));
}
