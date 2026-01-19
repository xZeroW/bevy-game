use bevy::prelude::*;
use crate::game::player::component::Player;
use crate::game::player::controls::controls;
use crate::game::player::controls::sync_position_transform;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update,
            (controls, sync_position_transform).chain());
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Player)
        .insert(
            Sprite::from_image(asset_server.load("classes/archer.png"))
        );

    commands.spawn((Sprite::from_image(
        asset_server.load("classes/default.png")
    ), Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))));
}
