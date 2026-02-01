use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::f32::consts::PI;
use std::time::Duration;

use rand::Rng;
use serde::Deserialize;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::game::config;
use crate::game::player::component::Player;
use crate::game::resources::GlobalTextureAtlas;
use crate::game::enemies::component::Enemy;
use crate::game::enemies::enemies::{EnemyType, CollidableEnemy};
use crate::game::animation::animation::{ AnimationTimer, AtlasIndex };

#[derive(Deserialize, Asset, TypePath)]
pub struct EnemyList {
    pub enemies: Vec<EnemySpec>,
}

#[derive(Deserialize)]
pub struct EnemySpec {
    pub x: f32,
    pub y: f32,
    pub kind: String,
    pub sprite: EnemySprite,
    pub metadata: EnemyMetadata,
}

#[derive(Deserialize)]
pub struct EnemySprite {
    pub path: String,
    pub idle: usize,
    pub moving: Vec<usize>,
    pub frame_time: Option<f32>,
    pub scale: Option<f32>,
}

#[derive(Deserialize, Resource)]
pub struct EnemyMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub health: Option<f32>,
    pub speed: Option<f32>,
    pub damage: Option<f32>,
    pub spawn_rate: f32,
}

#[derive(Resource)]
pub struct EnemyListHandle(pub Handle<EnemyList>);

pub struct EnemyAssetsExamplePlugin;

impl Plugin for EnemyAssetsExamplePlugin {
    fn build(&self, app: &mut App) {
        // register the RON loader for EnemyList files ending with `entities/enemies/devil.ron`
        app
            .add_plugins(RonAssetPlugin::<EnemyList>::new(&["devil.ron"]))
            .add_systems(Startup, setup)
            .add_systems(Update, spawn_enemies.run_if(on_timer(Duration::from_secs_f32(config::ENEMY_SPAWN_INTERVAL))));
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub enemy: Enemy,
    pub collidable: CollidableEnemy,
    pub atlas_index: AtlasIndex,
    pub enemy_type: EnemyType,
    pub timer: AnimationTimer,
}

impl EnemyBundle {
    pub fn from_spec(spec: &EnemySpec, atlas: &GlobalTextureAtlas, etype: EnemyType, pos: Vec3) -> Self {
        EnemyBundle {
            sprite: Sprite::from_atlas_image(
                atlas.image.clone(),
                TextureAtlas {
                    layout: atlas.layout.clone(),
                    index: spec.sprite.idle,
                },
            ),
            transform: Transform::from_translation(pos),
            enemy: Enemy::default(),
            collidable: CollidableEnemy::default(),
            atlas_index: AtlasIndex(spec.sprite.idle),
            enemy_type: etype,
            timer: AnimationTimer(Timer::from_seconds(spec.sprite.frame_time.unwrap_or(0.08), TimerMode::Repeating)),
        }
    }

    pub fn from_type(etype: EnemyType, atlas: &GlobalTextureAtlas, pos: Vec3, frame_time: f32) -> Self {
        let index = etype.get_base_sprite_index();
        EnemyBundle {
            sprite: Sprite::from_atlas_image(
                atlas.image.clone(),
                TextureAtlas {
                    layout: atlas.layout.clone(),
                    index,
                },
            ),
            transform: Transform::from_translation(pos),
            enemy: Enemy::default(),
            collidable: CollidableEnemy::default(),
            atlas_index: AtlasIndex(index),
            enemy_type: etype,
            timer: AnimationTimer(Timer::from_seconds(frame_time, TimerMode::Repeating)),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // this will actually look under the `assets/` folder at runtime
    let handle: Handle<EnemyList> = asset_server.load("entities/enemies/devil.ron");
    commands.insert_resource(EnemyListHandle(handle));
}

fn spawn_enemies(
    mut commands: Commands,
    enemylist_handles: Option<Res<EnemyListHandle>>,
    enemy_assets: Res<Assets<EnemyList>>,
    atlas: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    if num_enemies >= config::MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let enemy_spawn_count = (config::MAX_NUM_ENEMIES - num_enemies).min(config::SPAWN_RATE_PER_SECOND);

    let player_tf = if let Ok(t) = player_query.single() { t } else { return };
    let player_pos = player_tf.translation.truncate();

    // if we have an enemy list asset, use its spawn_rate weights to pick specs
    if let Some(handles) = enemylist_handles {
        if let Some(list) = enemy_assets.get(&handles.0) {
            // compute total spawn weight
            let mut total_weight: f32 = 0.0;
            for spec in &list.enemies {
                total_weight += spec.metadata.spawn_rate;
            }

            let mut rng = rand::rng();
            for _ in 0..enemy_spawn_count {
                // weighted random selection
                let mut pick = rng.random_range(0.0..total_weight);
                let mut chosen: &EnemySpec = &list.enemies[0];
                for spec in &list.enemies {
                    if pick <= spec.metadata.spawn_rate {
                        chosen = spec;
                        break;
                    }
                    pick -= spec.metadata.spawn_rate;
                }

                let (x, y) = get_random_position_around(player_pos);
                let etype = match chosen.kind.to_lowercase().as_str() {
                    "green" => EnemyType::Green,
                    "red" => EnemyType::Red,
                    _ => EnemyType::Skin,
                };

                commands.spawn(EnemyBundle::from_spec(chosen, &atlas, etype, Vec3::new(x, y, 1.0)));
            }
            return;
        }
    }

    // // fallback: random-type spawn if no asset available
    // let mut rng = rand::rng();
    // for _ in 0..enemy_spawn_count {
    //     let (x, y) = get_random_position_around(player_pos);
    //     let etype = match rng.random_range(0..3) {
    //         0 => EnemyType::Green,
    //         1 => EnemyType::Red,
    //         _ => EnemyType::Skin,
    //     };
    //     commands.spawn(EnemyBundle::from_type(etype, &atlas, Vec3::new(x, y, 1.0), 0.08));
    // }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..PI * 2.0);
    let dist = rng.random_range(750.0..1000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}
