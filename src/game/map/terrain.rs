use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::{HashMap, VecDeque};
use crate::game::config as cfg;

const CHUNK_W: u32 = cfg::CHUNK_W;
const CHUNK_H: u32 = cfg::CHUNK_H;
const TILE_WORLD_W: f32 = cfg::tile_world_w();
const TILE_WORLD_H: f32 = cfg::tile_world_h();
const SPACING_WORLD: f32 = cfg::spacing_world();
const RENDER_RADIUS: i32 = cfg::RENDER_RADIUS; // how many chunks away to generate
const NOISE_SCALE: f64 = cfg::NOISE_SCALE;

// Camera generation limit (clamped scale used for radius calc)
const MAX_GEN_SCALE: f32 = cfg::MAX_GEN_SCALE;

#[derive(Resource)]
struct GenerationSeed(u32);

const CACHE_CAPACITY: usize = 256; // max number of cached chunks

#[derive(Resource)]
struct ChunkCache {
    map: HashMap<(i32, i32), Vec<u8>>,
    order: VecDeque<(i32, i32)>,
    capacity: usize,
}

impl Default for ChunkCache {
    fn default() -> Self {
        ChunkCache { map: HashMap::new(), order: VecDeque::new(), capacity: CACHE_CAPACITY }
    }
}

impl ChunkCache {
    fn touch(&mut self, key: (i32, i32)) {
        // move key to back (most recently used)
        if let Some(pos) = self.order.iter().position(|k| *k == key) {
            self.order.remove(pos);
        }
        self.order.push_back(key);
    }

    fn insert(&mut self, key: (i32, i32), data: Vec<u8>) {
        if !self.map.contains_key(&key) {
            self.order.push_back(key);
        }
        self.map.insert(key, data);
        // evict oldest if over capacity
        while self.order.len() > self.capacity {
            if let Some(old) = self.order.pop_front() {
                self.map.remove(&old);
            }
        }
    }

    fn get_mut(&mut self, key: &(i32, i32)) -> Option<&mut Vec<u8>> {
        if self.map.contains_key(key) {
            self.touch(*key);
            self.map.get_mut(key)
        } else {
            None
        }
    }
}

#[derive(Resource, Default)]
struct CurrentChunks(HashMap<(i32, i32), Entity>);

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        // randomize seed at startup
        let seed: u32 = rand::random::<u32>();

        app.insert_resource(GenerationSeed(seed))
            .insert_resource(CurrentChunks::default())
            .insert_resource(ChunkCache::default())
            .add_plugins(TilemapPlugin)
            .add_systems(Update, generate_chunks_around_player)
            .add_systems(Update, cleanup_far_chunks)
            .add_systems(Update, update_ui);
    }
}



fn world_to_chunk_coord(x: f32, y: f32) -> (i32, i32) {
    let cx = (x / (CHUNK_W as f32 * TILE_WORLD_W)).floor() as i32;
    let cy = (y / (CHUNK_H as f32 * TILE_WORLD_H)).floor() as i32;
    (cx, cy)
}

fn generate_chunks_around_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut chunks: ResMut<CurrentChunks>,
    seed: Res<GenerationSeed>,
    mut cache: ResMut<ChunkCache>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    proj_query: Query<&Projection, With<Camera>>,
    player_query: Query<&Transform, With<crate::game::player::component::Player>>,
) {
    let player_tf = match player_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };
    let (pcx, pcy) = world_to_chunk_coord(player_tf.translation.x, player_tf.translation.y);

    let texture_handle = asset_server.load("sprite-sheet.png");
    // compute horizontal/vertical render radius in chunks based on window + camera
    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let mut scale = 1.0f32;
    if let Ok(proj) = proj_query.single() {
        if let Projection::Orthographic(ortho) = proj {
            // clamp to the allowed max+margin so generation covers a bit
            // more than the maximum zoom the player will use.
            scale = (ortho.scale as f32).min(MAX_GEN_SCALE);
        }
    }

    let half_w = window.width() * scale * 0.5;
    let half_h = window.height() * scale * 0.5;
    let chunk_world_w = CHUNK_W as f32 * TILE_WORLD_W;
    let chunk_world_h = CHUNK_H as f32 * TILE_WORLD_H;
    let render_radius_x = ((half_w / chunk_world_w).ceil() as i32).max(RENDER_RADIUS);
    let render_radius_y = ((half_h / chunk_world_h).ceil() as i32).max(RENDER_RADIUS);

    for dx in -render_radius_x..=render_radius_x {
        for dy in -render_radius_y..=render_radius_y {
            let cx = pcx + dx;
            let cy = pcy + dy;
            if chunks.0.contains_key(&(cx, cy)) {
                continue;
            }

            // obtain tile indices from cache or generate
            let key = (cx, cy);
            let tile_data: Vec<u8> = if let Some(stored) = cache.get_mut(&key) {
                stored.clone()
            } else {
                let data = generate_chunk_data(seed.0, key);
                cache.insert(key, data.clone());
                data
            };

            let tilemap_entity = spawn_chunk_from_data(&mut commands, texture_handle.clone(), (cx, cy), &tile_data);
            chunks.0.insert((cx, cy), tilemap_entity);
        }
    }
}

fn spawn_chunk_from_data(commands: &mut Commands, texture: Handle<Image>, chunk: (i32, i32), data: &[u8]) -> Entity {
    let size = TilemapSize { x: CHUNK_W, y: CHUNK_H };
    let grid_size = TilemapGridSize { x: TILE_WORLD_W, y: TILE_WORLD_H };
    let tile_size = TilemapTileSize { x: TILE_WORLD_W, y: TILE_WORLD_H };

    let mut storage = TileStorage::empty(size);
    let mut spawned: Vec<Entity> = Vec::new();

    for y in 0..CHUNK_H {
        for x in 0..CHUNK_W {
            let idx = (y * CHUNK_W + x) as usize;
            let tile_index = data.get(idx).copied().unwrap_or(0) as u32;
            let tile_pos = TilePos { x, y };
            let e = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId::default(),
                texture_index: TileTextureIndex(tile_index),
                ..Default::default()
            }).id();
            storage.set(&tile_pos, e);
            spawned.push(e);
        }
    }

    // position tilemap in world based on chunk coords
    let world_x = chunk.0 as f32 * CHUNK_W as f32 * TILE_WORLD_W;
    let world_y = chunk.1 as f32 * CHUNK_H as f32 * TILE_WORLD_H;

    let tilemap_entity = commands.spawn(TilemapBundle {
        grid_size,
        size,
        storage,
        texture: TilemapTexture::Single(texture),
        tile_size,
        spacing: TilemapSpacing { x: SPACING_WORLD, y: SPACING_WORLD },
        anchor: TilemapAnchor::TopLeft,
        transform: Transform::from_xyz(world_x, world_y, 0.0),
        ..Default::default()
    }).id();

    for e in spawned.iter() {
        // attach TilemapId to each tile entity
        commands.entity(*e).insert(TilemapId(tilemap_entity));
    }

    // tiles store `TilemapId`, so we can find and despawn them when
    // the parent tilemap is removed.

    tilemap_entity
}

fn generate_chunk_data(gen_seed: u32, chunk: (i32, i32)) -> Vec<u8> {
    let perlin = Perlin::new(gen_seed);
    let mut data = Vec::with_capacity((CHUNK_W * CHUNK_H) as usize);
    for y in 0..CHUNK_H {
        for x in 0..CHUNK_W {
            let global_x = chunk.0 * CHUNK_W as i32 + x as i32;
            let global_y = chunk.1 * CHUNK_H as i32 + y as i32;
            let nx = global_x as f64 / NOISE_SCALE;
            let ny = global_y as f64 / NOISE_SCALE;
            let noise_val = perlin.get([nx, ny]);
            let tile_index = if noise_val < -0.1 { 0 } else if noise_val < 0.4 { 1 } else { 2 };
            data.push(tile_index as u8);
        }
    }
    data
}

fn cleanup_far_chunks(
    mut commands: Commands,
    mut chunks: ResMut<CurrentChunks>,
    tiles_query: Query<(Entity, &TilemapId)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    proj_query: Query<&Projection, With<Camera>>,
    player_query: Query<&Transform, With<crate::game::player::component::Player>>,
) {
    let player_tf = match player_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };
    let (pcx, pcy) = world_to_chunk_coord(player_tf.translation.x, player_tf.translation.y);

    // compute render radii to determine which chunks are far
    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let mut scale = 1.0f32;
    if let Ok(proj) = proj_query.single() {
        if let Projection::Orthographic(ortho) = proj {
            scale = (ortho.scale as f32).min(MAX_GEN_SCALE);
        }
    }
    let half_w = window.width() * scale * 0.5;
    let half_h = window.height() * scale * 0.5;
    let chunk_world_w = CHUNK_W as f32 * TILE_WORLD_W;
    let chunk_world_h = CHUNK_H as f32 * TILE_WORLD_H;
    let render_radius_x = ((half_w / chunk_world_w).ceil() as i32).max(RENDER_RADIUS);
    let render_radius_y = ((half_h / chunk_world_h).ceil() as i32).max(RENDER_RADIUS);

    let mut to_remove: Vec<(i32, i32)> = Vec::new();
    for (&(cx, cy), &entity) in chunks.0.iter() {
        let dx = cx - pcx;
        let dy = cy - pcy;
            if dx.abs() > render_radius_x + 1 || dy.abs() > render_radius_y + 1 {
            // find and despawn all tile entities that reference this tilemap
            let mut tiles_to_kill: Vec<Entity> = Vec::new();
            for (te, tt) in tiles_query.iter() {
                if tt.0 == entity {
                    tiles_to_kill.push(te);
                }
            }
            for te in tiles_to_kill {
                commands.entity(te).despawn();
            }

            // now despawn the tilemap entity itself
            commands.entity(entity).despawn();
            to_remove.push((cx, cy));
        }
    }
    for k in to_remove {
        chunks.0.remove(&k);
    }
}

fn update_ui(chunks: Res<CurrentChunks>, seed: Res<GenerationSeed>, cache: Res<ChunkCache>, mut windows: Query<&mut Window>) {
    let chunk_count = chunks.0.len();
    let seed_value = seed.0;
    let cache_size = cache.map.len();
    let text_value = format!("Chunks: {}   Seed: {}   Cache: {}", chunk_count, seed_value, cache_size);

    if let Ok(mut window) = windows.single_mut() {
        window.title = text_value;
    }
}

