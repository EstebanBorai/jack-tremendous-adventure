pub mod animation;

use bevy::prelude::{
    AssetServer, Assets, Commands, Component, Entity, Handle, Input, KeyCode, Query, Res, ResMut,
    Transform, Vec2, With, Without,
};
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};
use bevy::time::Time;

use crate::component::jump::{Jump, FALL_SPEED};
use crate::component::speed::Speed;
use crate::component::sprite_animation::{FrameTime, SpriteAnimation};
use crate::resources::image::MASK_DUDE_IDLE_32X32;

const REGULAR_JUMP_FORCE: f32 = 25.;

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
            SpriteAnimation::new(11, 1. / 20.),
            FrameTime(0.),
            Speed(100.),
        ));
    }

    pub fn movement(
        mut commands: Commands,
        mut player: Query<(Entity, &mut Transform, &Speed), With<Player>>,
        time: Res<Time>,
        input: Res<Input<KeyCode>>,
    ) {
        let player = player.single_mut();
        let (player, mut transform, speed) = player;

        if input.any_pressed(MOVEMENT_KEYS) {
            let direction =
                if input.pressed(MOVEMENT_LEFT_KEYS[0]) || input.pressed(MOVEMENT_LEFT_KEYS[1]) {
                    -1.
                } else {
                    1.
                };

            transform.translation.x += time.delta_seconds() * speed.0 * direction;
        }

        // If the player is in the groaund and the user presses the space bar
        // sets the Jump component
        if input.pressed(KeyCode::Space) && transform.translation.y == 0. {
            commands.entity(player).insert(Jump(REGULAR_JUMP_FORCE));
        }
    }

    pub fn jump(
        mut commands: Commands,
        time: Res<Time>,
        mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
    ) {
        // Get player only if it has a jump component, `Jump` component is available
        // only when the user is jumping.
        if let Ok(player) = player.get_single_mut() {
            let (entity, mut transform, mut jump) = player;
            let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);

            jump.0 -= jump_power;
            transform.translation.y += jump_power;

            if jump.0 == 0. {
                commands.entity(entity).remove::<Jump>();
            }
        }
    }

    pub fn fall(mut player: Query<&mut Transform, (With<Player>, Without<Jump>)>, time: Res<Time>) {
        if let Ok(mut player) = player.get_single_mut() {
            // If the player is in the air we reduce the `y` position by `FALL_SPEED`
            // on every frame until it reaches the ground. (0.)
            if player.translation.y > 0. {
                player.translation.y -= time.delta_seconds() * FALL_SPEED;

                if player.translation.y < 0. {
                    player.translation.y = 0.;
                }
            }
        }
    }

    pub fn update_player_animation(
        mut player: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite), With<Player>>,
        input: Res<Input<KeyCode>>,
        animations: Res<animation::PlayerAnimation>,
    ) {
        let (mut atlas, mut sprite) = player.single_mut();

        if input.any_just_pressed(MOVEMENT_KEYS) {
            let (next_atlas, _) = animations.get(animation::Animation::Run);

            *atlas = next_atlas;

            if input.any_pressed(MOVEMENT_RIGHT_KEYS) {
                sprite.flip_x = false;
            } else if input.any_pressed(MOVEMENT_LEFT_KEYS) {
                sprite.flip_x = true;
            }
        }

        if input.any_just_released(MOVEMENT_KEYS) || input.just_released(KeyCode::Space) {
            let (next_atlas, _) = animations.get(animation::Animation::Idle);

            *atlas = next_atlas;
        }

        if input.pressed(KeyCode::Space) {
            let (next_atlas, _) = animations.get(animation::Animation::Jump);

            *atlas = next_atlas;
        }

        if input.just_released(KeyCode::Space) {
            let (next_atlas, _) = animations.get(animation::Animation::Fall);

            *atlas = next_atlas;
        }
    }
}
