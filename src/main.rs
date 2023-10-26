mod components;
mod resources;

use bevy::prelude::*;

use components::{player::spawn_player, sprite_animation::animate_sprite};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .add_systems(Update, animate_sprite)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
