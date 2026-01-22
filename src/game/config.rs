use bevy::prelude::*;

// Shared game configuration constants.
// Keep these in one place so other systems (camera, terrain) can reference them.

// Sprite / tiles
pub const SPRITE_PIX_W: u32 = 6;
pub const SPRITE_PIX_H: u32 = 8;
pub const SPRITE_SCALE: u32 = 5;
pub const SPRITE_PADDING: f32 = 2.0; // pixels in the sheet

// Chunk layout
pub const CHUNK_W: u32 = 16;
pub const CHUNK_H: u32 = 16;

// Noise
pub const NOISE_SCALE: f64 = 60.0;

// Camera generation limits
pub const ORTHO_MIN_SCALE: f32 = 0.5;
pub const ORTHO_MAX_SCALE: f32 = 1.5;
pub const ORTHO_GEN_MARGIN: f32 = 0.5;
pub const MAX_GEN_SCALE: f32 = ORTHO_MAX_SCALE + ORTHO_GEN_MARGIN;

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
