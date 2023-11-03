use bevy::prelude::{
    AssetServer, Assets, Commands, Component, Input, KeyCode, Query, Res, ResMut, Transform, Vec2,
    With,
};
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};
use bevy::time::Time;

use crate::component::speed::Speed;
use crate::component::sprite_animation::{FrameTime, SpriteAnimation};
use crate::resources::image::MASK_DUDE_IDLE_32X32;

// FIXME: This shouild be `A`` and `D` for Qwerty! Im using Dvorak so I use `A` and `E` instead.
const MOVEMENT_RIGHT_KEYS: [KeyCode; 2] = [KeyCode::Right, KeyCode::E];
const MOVEMENT_LEFT_KEYS: [KeyCode; 2] = [KeyCode::Left, KeyCode::A];
const MOVEMENT_KEYS: [KeyCode; 4] = [
    MOVEMENT_RIGHT_KEYS[0],
    MOVEMENT_RIGHT_KEYS[1],
    MOVEMENT_LEFT_KEYS[0],
    MOVEMENT_LEFT_KEYS[1],
];

#[derive(Debug, Component)]
pub struct Player;

impl Player {
    pub fn spawn(
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
            Speed(100.),
        ));
    }

    pub fn movement(
        mut player: Query<(&mut Transform, &Speed), With<Player>>,
        time: Res<Time>,
        input: Res<Input<KeyCode>>,
    ) {
        let player = player.single_mut();
        let (mut transform, speed) = player;

        if input.any_pressed(MOVEMENT_KEYS) {
            let direction =
                if input.pressed(MOVEMENT_LEFT_KEYS[0]) || input.pressed(MOVEMENT_LEFT_KEYS[1]) {
                    -1.
                } else {
                    1.
                };

            transform.translation.x += time.delta_seconds() * speed.0 * direction;
        }
    }
}
