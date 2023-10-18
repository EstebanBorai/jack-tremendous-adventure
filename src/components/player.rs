use bevy::{
    ecs::system::Command,
    prelude::{AssetServer, Commands, Handle, Res, Vec2, Component, ResMut, Assets},
    sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite},
    text,
};

const PLAYER_ASSET_PATH: &str = "Main Characters/Mask Dude/Idle (32x32).png";

#[derive(Debug, Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load(PLAYER_ASSET_PATH),
        Vec2::splat(32.),
        11,
        1,
        None,
        None,
    );

    commands.spawn(SpriteSheetBundle {
      texture_atlas: texture_atlas.add(atlas),
      sprite: TextureAtlasSprite::new(0),
      ..Default::default()
    });
    commands.spawn(Player);
}
