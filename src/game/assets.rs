use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::enemies::enemies_example::EnemyList;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "tilesets/assets.png")]
    pub sprite_sheet: Handle<Image>,

    #[asset(path = "entities/enemies/devil.ron")]
    pub enemy_list: Handle<EnemyList>,
}
