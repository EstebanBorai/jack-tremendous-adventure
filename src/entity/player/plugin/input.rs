use bevy::prelude::{
    App, Commands, Entity, Input, KeyCode, Plugin, Query, Res, Transform, Update, With,
};
use bevy::time::Time;

use crate::component::jump::Jump;
use crate::component::speed::Speed;
use crate::entity::player::Player;

// FIXME: This shouild be `A`` and `D` for Qwerty! Im using Dvorak so I use `A` and `E` instead.
const MOVEMENT_RIGHT_KEYS: [KeyCode; 2] = [KeyCode::Right, KeyCode::E];
const MOVEMENT_LEFT_KEYS: [KeyCode; 2] = [KeyCode::Left, KeyCode::A];
const MOVEMENT_KEYS: [KeyCode; 4] = [
  MOVEMENT_RIGHT_KEYS[0],
  MOVEMENT_RIGHT_KEYS[1],
  MOVEMENT_LEFT_KEYS[0],
  MOVEMENT_LEFT_KEYS[1],
];

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}

pub enum PlayerAction {
    Fall,
    Jump,
    WalkRight,
    WalkLeft,
}

fn handle_input(
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
        commands.entity(player).insert(Jump(95.));
    }
}
