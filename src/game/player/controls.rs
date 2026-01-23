use bevy::{ecs::{entity::Entity, query::With, system::Commands}, prelude::{ButtonInput, KeyCode, Query, Res, Time, Transform}, sprite::Sprite, window::Window};

use crate::game::{common::components::characters::{move_speed::MoveSpeed, position::Position}, player::component::Player};

pub fn controls(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Query<&MoveSpeed, With<Player>>,
    mut query: Query<(&mut Position, &mut Sprite)>
) {
    let speed: f32 = match player.single() {
        Ok(ms) => ms.0 as f32,
        Err(_) => 300.0,
    };

    for (mut pos, mut sprite) in &mut query {
        if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
            sprite.flip_x = false;
            pos.x += speed * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            sprite.flip_x = true;
            pos.x -= speed * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
            pos.y += speed * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
            pos.y -= speed * time.delta_secs();
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
