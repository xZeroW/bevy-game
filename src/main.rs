use bevy::{prelude::*, window::PresentMode};
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub mod game;

use crate::game::{game::GamePlugin, ui::fps::FpsPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bevy Game"),
                        present_mode: PresentMode::Immediate,
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((GamePlugin, FpsPlugin))
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}
