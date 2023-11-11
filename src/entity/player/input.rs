use bevy::prelude::{
    App, Commands, Entity, Input, KeyCode, Plugin, Query, Res, Transform, Update, With, Without,
};
use bevy::time::Time;

use crate::component::jump::{Jump, FALL_SPEED};
use crate::component::run::{Run, RunDirection};
use crate::component::speed::Speed;
use crate::component::walk::Walk;
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
        app.add_systems(Update, (handle_input, fall, jump, run, walk));
    }
}

fn handle_input(
    mut commands: Commands,
    mut player: Query<Entity, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let player = player.single_mut();

    if input.any_pressed(MOVEMENT_KEYS) && input.pressed(KeyCode::R) {
        let run_direction =
            if input.pressed(MOVEMENT_LEFT_KEYS[0]) || input.pressed(MOVEMENT_LEFT_KEYS[1]) {
                RunDirection::Left
            } else {
                RunDirection::Right
            };

        commands.entity(player).insert(Run {
            speed: 150.,
            direction: run_direction,
        });
    }

    if input.any_pressed(MOVEMENT_KEYS) {
        let walk_action =
            if input.pressed(MOVEMENT_LEFT_KEYS[0]) || input.pressed(MOVEMENT_LEFT_KEYS[1]) {
                Walk::Left
            } else {
                Walk::Right
            };

        commands.entity(player).insert(walk_action);
    }

    // If the player is in the groaund and the user presses the space bar
    // sets the Jump component
    if input.pressed(KeyCode::Space) {
        commands.entity(player).insert(Jump(95.));
    }
}

pub fn run(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Query<(Entity, &mut Transform, &Run), With<Player>>,
) {
    if let Ok(player) = player.get_single_mut() {
        let (entity, mut transform, run) = player;
        match run.direction {
            RunDirection::Left => {
                transform.translation.x += time.delta_seconds() * run.speed * -1.;
            }
            RunDirection::Right => {
                transform.translation.x += time.delta_seconds() * run.speed * 1.;
            }
        }

        commands.entity(entity).remove::<Run>();
    }
}

pub fn walk(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Query<(Entity, &mut Transform, &Walk, &Speed), With<Player>>,
) {
    if let Ok(player) = player.get_single_mut() {
        let (entity, mut transform, walk, speed) = player;
        match walk {
            Walk::Left => {
                transform.translation.x += time.delta_seconds() * speed.0 * -1.;
            }
            Walk::Right => {
                transform.translation.x += time.delta_seconds() * speed.0 * 1.;
            }
        }

        commands.entity(entity).remove::<Walk>();
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
