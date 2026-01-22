use bevy::prelude::*;

use crate::game::map::terrain::TerrainPlugin;
use crate::game::player::player::PlayerPlugin;
// use crate::game::map::map::MapPlugin;
use crate::game::camera::camera::CameraPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((PlayerPlugin, CameraPlugin, TerrainPlugin))
        .insert_resource(ClearColor(Color::srgb(0.4, 0.35, 0.45)));
    }
}


