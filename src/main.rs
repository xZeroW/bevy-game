use bevy::{prelude::*, remote::{RemotePlugin, http::RemoteHttpPlugin}};

pub mod game;
use crate::game::{game::GamePlugin, player::component::{MoveSpeed, PlayerBundle, PlayerState, Position}};

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
        .add_systems(Startup, setup)
        .add_plugins(GamePlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(PlayerBundle {
        sprite: Sprite::from_image(asset_server.load("classes/default.png")),
        position: Position { x: 0., y: 0. },
        move_speed: MoveSpeed(150),
        state: PlayerState::Idle,
    });

    commands.spawn((
        Sprite::from_image(asset_server.load("classes/default.png")),
    ));
}
