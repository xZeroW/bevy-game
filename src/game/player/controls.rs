use bevy::{ecs::{entity::Entity, system::Commands}, prelude::{ButtonInput, KeyCode, Query, Res, Time, Transform}, sprite::Sprite, window::Window};

use crate::game::common::components::characters::position::Position;

pub fn controls(input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut query: Query<(&mut Position, &mut Sprite)>) {
    for (mut pos, mut sprite) in &mut query {
        if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
            sprite.flip_x = false;
            pos.x += 300.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            sprite.flip_x = true;
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
        println!("Window focus: {}", focus.focused);
        println!("Input state: {:?}", input);
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            println!("Escape pressed, closing window");
            commands.entity(window).despawn();
        }
    }
}
