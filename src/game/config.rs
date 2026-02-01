use bevy::prelude::*;

// Shared game configuration constants.
// Keep these in one place so other systems (camera, terrain) can reference them.

// Sprite / tiles
pub const SPRITE_SHEET_PATH: &str = "tilesets/assets.png";
pub const SPRITE_PIX_W: u32 = 8;
pub const SPRITE_PIX_H: u32 = 10;
pub const SPRITE_SCALE: u32 = 1;
pub const SPRITE_PADDING: f32 = 1.0; // pixels in the sheet
pub const TILE_W: u32 = 16;
pub const TILE_H: u32 = 16;
pub const SPRITE_SHEET_W: u32 = 8;
pub const SPRITE_SHEET_H: u32 = 8;

// Chunk layout
pub const CHUNK_W: u32 = 16;
pub const CHUNK_H: u32 = 16;

// Noise
pub const NOISE_SCALE: f64 = 60.0;

// Camera generation limits
pub const ORTHO_MIN_SCALE: f32 = 0.2;
pub const ORTHO_MAX_SCALE: f32 = 0.5;
pub const ORTHO_GEN_MARGIN: f32 = 0.5;
pub const MAX_GEN_SCALE: f32 = ORTHO_MAX_SCALE + ORTHO_GEN_MARGIN;

// Gun
pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;
pub const BULLET_TIME_SECS: f32 = 1.;
pub const BULLET_SPEED: f32 = 500.0;
pub const BULLET_DAMAGE: f32 = 15.0;
pub const NUM_BULLETS_PER_SHOT: usize = 10;
pub const BULLET_SPREAD: f32 = 0.5;

// Enemy
pub const ENEMY_HEALTH: f32 = 30.0;
pub const ENEMY_SPEED: f32 = 50.0;
pub const MAX_NUM_ENEMIES: usize = 50_000;
pub const SPAWN_RATE_PER_SECOND: usize = 2;
pub const ENEMY_SPAWN_INTERVAL: f32 = 0.5;
pub const ENEMY_DAMAGE: f32 = 10.0;

// Kd-tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.1;

// Render radius fallback (in chunks) if window isn't available
pub const RENDER_RADIUS: i32 = 3;

// Extra vertical padding (in chunks) to generate above/below the viewport.
// Increase this if vertical generation feels too tight on tall/ultrawide displays.
pub const VERTICAL_GEN_MARGIN: i32 = 1;

// Derived values helpers
pub const fn tile_world_w() -> f32 {
    (SPRITE_PIX_W * SPRITE_SCALE) as f32
}

pub const fn tile_world_h() -> f32 {
    (SPRITE_PIX_H * SPRITE_SCALE) as f32
}

pub const fn spacing_world() -> f32 {
    SPRITE_PADDING * SPRITE_SCALE as f32
}

// Useful resource to expose config at runtime if needed in future
#[derive(Resource, Clone, Copy)]
pub struct GameConfig;

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig
    }
}
