use bevy::{
    prelude::{
        App, AssetServer, Assets, Bundle, FromWorld, Handle, Input, KeyCode, Mut, Plugin, Query,
        Res, Resource, Update, Vec2, With,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
    time::Time,
    utils::HashMap,
};

use crate::{
    component::sprite_animation::{FrameTime, SpriteAnimation},
    entity::player::Player,
};

use super::input::{MOVEMENT_KEYS, MOVEMENT_LEFT_KEYS, MOVEMENT_RIGHT_KEYS};

const JACK_SPRITE_SETS_PATH: &str = "Characters/Jack";

#[derive(Bundle)]
pub struct PlayerAnimationBundle {
    pub animation: SpriteAnimation,
    pub frame_time: FrameTime,
}

impl PlayerAnimationBundle {
    pub fn new(animation: SpriteAnimation) -> Self {
        Self {
            animation,
            frame_time: FrameTime(20.),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Animation {
    Bark,
    Idle,
    Run,
    Walk,
}

#[derive(Resource)]
pub struct PlayerAnimation {
    map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
}

impl PlayerAnimation {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    pub fn set(
        &mut self,
        id: Animation,
        handle: Handle<TextureAtlas>,
        sprite_animation: SpriteAnimation,
    ) {
        self.map.insert(id, (handle, sprite_animation));
    }

    pub fn get(&self, id: Animation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
        self.map.get(&id).cloned()
    }
}

impl FromWorld for PlayerAnimation {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut player_animation = PlayerAnimation::new();

        world.resource_scope(|world, mut texture_atlas: Mut<Assets<TextureAtlas>>| {
            let asset_server = world.resource::<AssetServer>();

            // Bark Animation Atlas
            {
                let bark_atlas = TextureAtlas::from_grid(
                    asset_server.load(format!("{JACK_SPRITE_SETS_PATH}/Bark.png")),
                    Vec2::new(90., 57.),
                    4,
                    1,
                    Some(Vec2::new(0., 0.)),
                    None,
                );

                player_animation.set(
                    Animation::Bark,
                    texture_atlas.add(bark_atlas),
                    SpriteAnimation::new(4, 20),
                );
            }

            // Idle Animation Atlas
            {
                let idle_atlas = TextureAtlas::from_grid(
                    asset_server.load(format!("{JACK_SPRITE_SETS_PATH}/Idle.png")),
                    Vec2::new(90., 57.),
                    1,
                    1,
                    Some(Vec2::new(4., 0.)),
                    None,
                );

                player_animation.set(
                    Animation::Idle,
                    texture_atlas.add(idle_atlas),
                    SpriteAnimation::new(1, 1),
                );
            }

            // Run Animation Atlas
            {
                let run_atlas = TextureAtlas::from_grid(
                    asset_server.load(format!("{JACK_SPRITE_SETS_PATH}/Run.png")),
                    Vec2::new(90., 57.),
                    5,
                    1,
                    Some(Vec2::new(0., 0.)),
                    None,
                );

                player_animation.set(
                    Animation::Run,
                    texture_atlas.add(run_atlas),
                    SpriteAnimation::new(5, 20),
                );
            }

            // Walk Animation Atlas
            {
                let walk_atlas = TextureAtlas::from_grid(
                    asset_server.load(format!("{JACK_SPRITE_SETS_PATH}/Walk.png")),
                    Vec2::new(90., 57.),
                    6,
                    1,
                    Some(Vec2::new(0., 0.)),
                    None,
                );

                player_animation.set(
                    Animation::Walk,
                    texture_atlas.add(walk_atlas),
                    SpriteAnimation::new(6, 20),
                );
            }
        });

        player_animation
    }
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_sprite, update_animation))
            .init_resource::<PlayerAnimation>();
    }
}

fn animate_sprite(
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

fn update_animation(
    mut query: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
            &mut SpriteAnimation,
        ),
        With<Player>,
    >,
    input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimation>,
) {
    let (mut atlas, mut sprite, mut animation) = query.single_mut();

    if input.any_pressed(MOVEMENT_KEYS) && input.pressed(KeyCode::R) {
        let (next_atlas, next_animation) = animations.get(Animation::Run).unwrap();

        *atlas = next_atlas;
        sprite.index %= next_animation.len;
        *animation = next_animation;

        if input.any_pressed(MOVEMENT_RIGHT_KEYS) {
            sprite.flip_x = true;
        } else if input.any_pressed(MOVEMENT_LEFT_KEYS) {
            sprite.flip_x = false;
        }

        return;
    }

    if input.any_pressed(MOVEMENT_KEYS) {
        let (next_atlas, next_animation) = animations.get(Animation::Walk).unwrap();

        *atlas = next_atlas;
        sprite.index %= next_animation.len;
        *animation = next_animation;

        if input.any_pressed(MOVEMENT_RIGHT_KEYS) {
            sprite.flip_x = true;
        } else if input.any_pressed(MOVEMENT_LEFT_KEYS) {
            sprite.flip_x = false;
        }

        return;
    }

    let (next_atlas, next_animation) = animations.get(Animation::Idle).unwrap();

    *atlas = next_atlas;
    sprite.index %= next_animation.len;
    *animation = next_animation;
}
