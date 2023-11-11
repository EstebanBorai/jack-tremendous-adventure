mod animation;
mod input;

use bevy::{
    prelude::{error, App, Commands, Component, Name, Plugin, Res, Startup},
    reflect::Reflect,
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::component::{
    jump::Jump,
    run::{Run, RunDirection},
    speed::Speed,
    walk::Walk,
};

use self::animation::{Animation, PlayerAnimation, PlayerAnimationBundle, PlayerAnimationPlugin};
use self::input::PlayerInputPlugin;

#[derive(Debug, Component, PartialEq, Eq, Reflect)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimation>()
            .add_systems(Startup, spawn)
            .add_plugins((PlayerAnimationPlugin, PlayerInputPlugin))
            .register_type::<Player>();
    }
}

fn spawn(mut commands: Commands, animations: Res<PlayerAnimation>) {
    if let Some((texture_atlas, sprite_animation)) = animations.get(Animation::Idle) {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            },
            Player,
            Run {
                speed: 150.,
                direction: RunDirection::Right,
            },
            Speed(100.),
            PlayerAnimationBundle::new(sprite_animation),
            Jump(10.),
            Walk::Right,
            Name::new("Player"),
        ));
    } else {
        error!("Could not find the idle animation for the player!")
    }
}
