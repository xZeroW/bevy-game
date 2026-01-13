use bevy::prelude::{Query, Res, Time, Transform, ButtonInput, KeyCode};

use crate::game::player::component::Position;

pub fn controls(input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut query: Query<&mut Position>) {
    for mut pos in &mut query {
        if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
            pos.x += 300.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            pos.x -= 300.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
            pos.y += 300.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
            pos.y -= 300.0 * time.delta_secs();
        }
    }
}

pub fn sync_position_transform(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
