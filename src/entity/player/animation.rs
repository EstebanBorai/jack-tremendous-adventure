use bevy::{
    prelude::{AssetServer, Assets, FromWorld, Handle, Resource, Vec2},
    sprite::TextureAtlas,
    utils::HashMap,
};

use crate::component::sprite_animation::SpriteAnimation;

const MASK_DUDE_IDLE_32X32: &str = "Main Characters/Mask Dude/Idle (32x32).png";
const MASK_DUDE_RUN_32X32: &str = "Main Characters/Mask Dude/Run (32x32).png";

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Animation {
    Idle,
    Run,
}

#[derive(Resource)]
pub struct PlayerAnimation {
    map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
}

impl PlayerAnimation {
    pub fn set(
        &mut self,
        id: Animation,
        handle: Handle<TextureAtlas>,
        sprite_animation: SpriteAnimation,
    ) {
        self.map.insert(id, (handle, sprite_animation));
    }

    pub fn get(&self, id: Animation) -> (Handle<TextureAtlas>, SpriteAnimation) {
        self.map.get(&id).cloned().expect("Missing animation")
    }
}

impl FromWorld for PlayerAnimation {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut player_animation = PlayerAnimation {
            map: HashMap::default(),
        };
        let asset_server = world.resource::<AssetServer>();

        // Idle Animation Atlas
        let idle_atlas = TextureAtlas::from_grid(
            asset_server.load(MASK_DUDE_IDLE_32X32),
            Vec2::splat(32.),
            11,
            1,
            None,
            None,
        );

        // Run Animation Atlas
        let run_atlas = TextureAtlas::from_grid(
            asset_server.load(MASK_DUDE_RUN_32X32),
            Vec2::splat(32.),
            11,
            1,
            None,
            None,
        );

        let mut texture_atlas = world.resource_mut::<Assets<TextureAtlas>>();

        player_animation.set(
            Animation::Idle,
            texture_atlas.add(idle_atlas),
            SpriteAnimation {
                len: 11,
                frame_time: 1. / 20.,
            },
        );

        player_animation.set(
            Animation::Run,
            texture_atlas.add(run_atlas),
            SpriteAnimation {
                len: 12,
                frame_time: 1. / 20.,
            },
        );

        player_animation
    }
}
