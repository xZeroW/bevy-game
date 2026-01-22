use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::game::config as cfg;

use crate::game::player::component::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (zoom, sync_camera_position) );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn zoom(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera>>,
) {
    for event in mouse_wheel_events.read() {
        for mut projection in &mut query {
            let Projection::Orthographic(ortho) = &mut *projection else {
                continue;
            };

            if event.y < 0.0 {
                ortho.scale = (ortho.scale + 0.1).clamp(cfg::ORTHO_MIN_SCALE, cfg::ORTHO_MAX_SCALE);
            } else if event.y > 0.0 {
                ortho.scale = (ortho.scale - 0.1).clamp(cfg::ORTHO_MIN_SCALE, cfg::ORTHO_MAX_SCALE);
            }
        }
    }
}

fn sync_camera_position(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) = (player.single(), camera.single_mut()) {
        camera_transform.translation = player_transform.translation;
    }
}
