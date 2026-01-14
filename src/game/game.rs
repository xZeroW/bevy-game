use bevy::prelude::*;

use crate::game::player::player::PlayerPlugin;
use crate::game::player::component::Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
        .insert_resource(ClearColor(Color::srgb(0.4, 0.35, 0.45)))
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Player)
        .insert(Sprite::from_image(asset_server.load("classes/archer.png")));

    commands.spawn(Sprite::from_image(asset_server.load("classes/default.png")));
}
