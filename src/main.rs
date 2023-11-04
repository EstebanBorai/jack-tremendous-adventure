mod component;
mod entity;
mod resources;

use bevy::prelude::*;

use component::sprite_animation::animate_sprite;
use entity::player::{self, Player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, Player::spawn))
        .add_systems(
            Update,
            (
                animate_sprite,
                Player::movement,
                Player::jump,
                Player::fall,
                Player::update_player_animation,
            ),
        )
        .init_resource::<player::animation::PlayerAnimation>()
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
