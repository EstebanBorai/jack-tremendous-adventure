use bevy::prelude::{
    App, Commands, Entity, Input, KeyCode, Plugin, Query, Res, Transform, Update, With, Without,
};
use bevy::time::Time;

use crate::component::jump::{Jump, FALL_SPEED};
use crate::component::speed::Speed;
use crate::entity::player::Player;

// FIXME: This shouild be `A`` and `D` for Qwerty! Im using Dvorak so I use `A` and `E` instead.
pub(super) const MOVEMENT_RIGHT_KEYS: [KeyCode; 2] = [KeyCode::Right, KeyCode::E];
pub(super) const MOVEMENT_LEFT_KEYS: [KeyCode; 2] = [KeyCode::Left, KeyCode::A];
pub(super) const MOVEMENT_KEYS: [KeyCode; 4] = [
    MOVEMENT_RIGHT_KEYS[0],
    MOVEMENT_RIGHT_KEYS[1],
    MOVEMENT_LEFT_KEYS[0],
    MOVEMENT_LEFT_KEYS[1],
];

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_input, jump, fall));
    }
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
