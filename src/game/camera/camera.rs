use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update, zoom);
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

            if event.y > 0.0 {
                ortho.scale = (ortho.scale + 0.1).clamp(0.5, 1.5);
            } else if event.y < 0.0 {
                ortho.scale = (ortho.scale - 0.1).clamp(0.5, 1.5);
            }
        }
    }
}
