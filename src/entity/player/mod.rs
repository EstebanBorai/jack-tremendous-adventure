mod plugin;

use bevy::{
    prelude::{error, App, Commands, Component, Plugin, Res, Startup},
    reflect::Reflect,
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::component::speed::Speed;

use self::plugin::animation::{Animation, PlayerAnimation};
use self::plugin::input::PlayerInputPlugin;

#[derive(Debug, Component, PartialEq, Eq, Reflect)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerAnimation>()
            .add_systems(Startup, spawn)
            .add_plugins(PlayerInputPlugin)
            .register_type::<Player>();
    }
}

fn spawn(mut commands: Commands, animations: Res<PlayerAnimation>) {
    if let Some((texture_atlas, _)) =
        animations.get(Animation::Idle)
    {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            },
            Player,
            Speed(100.),
        ));
    } else {
        error!("Could not find the idle animation for the player!")
    }
}
