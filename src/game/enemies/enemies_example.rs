use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;

use bevy_common_assets::ron::RonAssetPlugin;

use crate::game::resources::GlobalTextureAtlas;
use crate::game::enemies::enemies::{Enemy, EnemyType};
use crate::game::player::atlas_index::AtlasIndex;
use crate::game::animation::animation::AnimationTimer;

#[derive(Deserialize, bevy::asset::Asset, TypePath)]
pub struct EnemyList {
    pub enemies: Vec<EnemySpec>,
}

#[derive(Deserialize)]
pub struct EnemySpec {
    pub x: f32,
    pub y: f32,
    pub kind: String,
    // optional per-enemy frame time
    pub frame_time: Option<f32>,
}

#[derive(Resource)]
pub struct EnemyListHandle(pub Handle<EnemyList>);

pub struct EnemyAssetsExamplePlugin;

impl Plugin for EnemyAssetsExamplePlugin {
    fn build(&self, app: &mut App) {
        // register the RON loader for EnemyList files ending with `enemies.ron`
        app.add_plugins(RonAssetPlugin::<EnemyList>::new(&["enemies.ron", "enemies.sample.ron"]))
            .add_systems(Startup, request_enemy_list)
            .add_systems(Update, spawn_enemies_from_asset);
    }
}

fn request_enemy_list(mut commands: Commands, asset_server: Res<AssetServer>) {
    // this will actually look under the `assets/` folder at runtime
    let handle: Handle<EnemyList> = asset_server.load("enemies.sample.ron");
    commands.insert_resource(EnemyListHandle(handle));
}

fn spawn_enemies_from_asset(
    mut commands: Commands,
    enemylist_handles: Option<Res<EnemyListHandle>>,
    enemy_assets: Res<Assets<EnemyList>>,
    atlas: Res<GlobalTextureAtlas>,
) {
    // bail out if there is no request for the asset
    let handles = if let Some(h) = enemylist_handles { h } else { return };

    if let Some(list) = enemy_assets.get(&handles.0) {
        for spec in &list.enemies {
            let etype = match spec.kind.to_lowercase().as_str() {
                "green" => EnemyType::Green,
                "red" => EnemyType::Red,
                _ => EnemyType::Skin,
            };

            commands.spawn((
                Sprite::from_atlas_image(
                    atlas.image.clone(),
                    TextureAtlas {
                        layout: atlas.layout.clone(),
                        index: etype.get_base_sprite_index(),
                    },
                ),
                Transform::from_translation(Vec3::new(spec.x, spec.y, 1.0)),
                Enemy::default(),
                AtlasIndex(0),
                etype,
                AnimationTimer(Timer::from_seconds(spec.frame_time.unwrap_or(0.08), TimerMode::Repeating)),
            ));
        }

        // remove the handle resource so we don't respawn every frame
        commands.remove_resource::<EnemyListHandle>();
    }
}
