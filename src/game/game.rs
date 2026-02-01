use bevy::prelude::*;

use crate::game::enemies::enemies::EnemyPlugin;
use crate::game::enemies::enemies_example::EnemyAssetsExamplePlugin;
// use crate::game::map::terrain::TerrainPlugin;
use crate::game::player::player::PlayerPlugin;
use crate::game::common::components::characters::health::HealthPlugin;
// use crate::game::map::map::MapPlugin;
use crate::game::camera::camera::CameraPlugin;
use crate::game::resources::ResourcesPlugin;
use crate::game::collisions::collisions::CollisionPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((ResourcesPlugin, HealthPlugin, PlayerPlugin, CameraPlugin, EnemyPlugin, CollisionPlugin, EnemyAssetsExamplePlugin));
    }
}
