use bevy::{prelude::*, remote::{RemotePlugin, http::RemoteHttpPlugin}};

pub mod game;

use crate::game::player::component::Player;
use crate::game::common::components::characters::{move_speed::MoveSpeed, position::Position, state::State};
use crate::game::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bevy Platformer"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.4, 0.35, 0.45)))
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Player {
        sprite:Sprite::from_image(asset_server.load("classes/archer.png")),
        position: Position::default(),
        move_speed: MoveSpeed::default(),
        state: State::default(),
    });

    commands.spawn((
        Sprite::from_image(asset_server.load("classes/default.png")),
    ));
}
