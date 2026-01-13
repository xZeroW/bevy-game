use bevy::{prelude::*, remote::{RemotePlugin, http::RemoteHttpPlugin}};

use crate::game::player::player::PlayerPlugin;
use crate::game::player::component::Player;
use crate::game::common::components::characters::{move_speed::MoveSpeed, position::Position, state::State};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
        .insert_resource(ClearColor(Color::srgb(0.4, 0.35, 0.45)))
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_systems(Startup, setup);
    }
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
