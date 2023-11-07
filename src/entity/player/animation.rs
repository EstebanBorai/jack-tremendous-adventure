use bevy::{
    prelude::{
        error, App, AssetServer, Assets, Bundle, FromWorld, Handle, Mut, Plugin, Query, Res,
        Resource, Update, Vec2, With,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
    time::Time,
    utils::HashMap,
};

use crate::{
    component::{
        jump::Jump,
        sprite_animation::{FrameTime, SpriteAnimation},
    },
    entity::player::Player,
};

const MASK_DUDE_IDLE_32X32: &str = "Main Characters/Mask Dude/Idle (32x32).png";
const MASK_DUDE_RUN_32X32: &str = "Main Characters/Mask Dude/Run (32x32).png";
const MASK_DUDE_JUMP_32X32: &str = "Main Characters/Mask Dude/Jump (32x32).png";
const MASK_DUDE_FALL_32X32: &str = "Main Characters/Mask Dude/Fall (32x32).png";

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                PlayerAnimationPlugin::animate_sprite,
                PlayerAnimationPlugin::animate_player,
            ),
        )
        .init_resource::<PlayerAnimation>();
    }
}

impl PlayerAnimationPlugin {
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

    fn animate_player(
        mut player: Query<
            (
                &Player,
                &mut Handle<TextureAtlas>,
                &mut SpriteAnimation,
                &mut TextureAtlasSprite,
                &Jump,
            ),
            With<Player>,
        >,
        animaitons: Res<PlayerAnimation>,
    ) {
        let (_player, mut atlas, mut animation, mut sprite, _jump) = player.single_mut();
        let Some((new_atlas, new_animaiton)) = animaitons.get(Animation::Fall) else {
            error!("No Animation Jump Loaded");
            return;
        };

        *atlas = new_atlas;
        sprite.index %= new_animaiton.len;
        *animation = new_animaiton;
    }
}

#[derive(Bundle)]
pub struct AnimationBundle {
    pub animation: SpriteAnimation,
    pub frame_time: FrameTime,
}

impl AnimationBundle {
    pub fn new(animation: SpriteAnimation) -> Self {
        Self {
            animation,
            frame_time: FrameTime(0.),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Animation {
    Idle,
    Run,
    Jump,
    Fall,
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

            // Idle Animation Atlas
            {
                let idle_atlas = TextureAtlas::from_grid(
                    asset_server.load(MASK_DUDE_IDLE_32X32),
                    Vec2::splat(32.),
                    11,
                    1,
                    None,
                    None,
                );

                player_animation.set(
                    Animation::Idle,
                    texture_atlas.add(idle_atlas),
                    SpriteAnimation::new(11, 20),
                );
            }

            // Run Animation Atlas
            {
                let run_atlas = TextureAtlas::from_grid(
                    asset_server.load(MASK_DUDE_RUN_32X32),
                    Vec2::splat(32.),
                    11,
                    1,
                    None,
                    None,
                );

                player_animation.set(
                    Animation::Run,
                    texture_atlas.add(run_atlas),
                    SpriteAnimation::new(12, 20),
                );
            }

            // Jump Animation Atlas
            {
                let jump_atlas = TextureAtlas::from_grid(
                    asset_server.load(MASK_DUDE_JUMP_32X32),
                    Vec2::splat(32.),
                    1,
                    1,
                    None,
                    None,
                );

                player_animation.set(
                    Animation::Jump,
                    texture_atlas.add(jump_atlas),
                    SpriteAnimation::new(1, 1),
                );
            }

            // Fall Animation Atlas
            {
                let fall = TextureAtlas::from_grid(
                    asset_server.load(MASK_DUDE_FALL_32X32),
                    Vec2::splat(32.),
                    1,
                    1,
                    None,
                    None,
                );
                player_animation.set(
                    Animation::Fall,
                    texture_atlas.add(fall),
                    SpriteAnimation::new(1, 1),
                );
            }
        });

        player_animation
    }
}
