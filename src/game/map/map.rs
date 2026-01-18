use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

use crate::game::helpers;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(TilemapPlugin)
        .add_plugins(helpers::tiled::TiledMapPlugin)
        .add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle = helpers::tiled::TiledMapHandle(asset_server.load("maps/map1.tmx"));

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
