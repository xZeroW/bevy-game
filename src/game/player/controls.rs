use bevy::{ecs::{entity::Entity, query::With, system::Commands}, prelude::{ButtonInput, KeyCode, Query, Res, Time, Transform, Vec2}, sprite::Sprite, window::Window};

use crate::game::{common::components::characters::{move_speed::MoveSpeed, position::Position, char_state::State}, player::component::Player};

pub fn controls(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Query<&MoveSpeed, With<Player>>,
    mut query: Query<(&mut Position, &mut Sprite, &mut State), With<Player>>,
) {
    let speed: f32 = match player.single() {
        Ok(ms) => ms.0 as f32,
        Err(_) => 300.0,
    };

    for (mut pos, mut sprite, mut state) in &mut query {
        let mut input_dir = Vec2::ZERO;
        if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
            input_dir.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            input_dir.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
            input_dir.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
            input_dir.y -= 1.0;
        }

        if input_dir != Vec2::ZERO {
            let dt = time.delta().as_secs_f32();
            let dir = input_dir.normalize();
            pos.x += dir.x * speed * dt;
            pos.y += dir.y * speed * dt;
            *state = State::Moving;

            // sprite flip based on horizontal input
            if dir.x > 0.0 {
                sprite.flip_x = false;
            } else if dir.x < 0.0 {
                sprite.flip_x = true;
            }
        } else {
            *state = State::Idle;
        }
    }
}

pub fn sync_position_transform(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        transform.translation.z = 1.0;
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
