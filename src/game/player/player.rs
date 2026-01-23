use bevy::prelude::*;
use crate::game::player::component::Player;
use crate::game::player::controls::{controls, sync_position_transform, close_on_esc};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update,
            (controls, close_on_esc))
        .add_systems(FixedUpdate, sync_position_transform);
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Player)
        .insert(
            Sprite::from_image(asset_server.load("classes/archer.png")),
        )
        .insert(Name::new("Player"));
}
