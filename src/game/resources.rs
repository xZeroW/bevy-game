use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::config as cfg;
use crate::game::game_state::GameState;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(Startup, load_assets)
            .add_systems(
                Update,
                update_cursor_position,
            );
    }
}

pub fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    println!("Loading global texture atlas...");
    handle.image = asset_server.load(cfg::SPRITE_SHEET_PATH);

    let tile_w = cfg::TILE_W;
    let tile_h = cfg::TILE_H;

    if tile_w == 0 || tile_h == 0 {
        return;
    }

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(tile_w, tile_h),
        cfg::SPRITE_SHEET_W,
        cfg::SPRITE_SHEET_H,
        None,
        None,
    );

    handle.layout = layouts.add(layout);
    next_state.set(GameState::InGame);
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection), With<Camera>>,
) {
    // get camera and window; bail out if either is unavailable
    let (camera, camera_transform, projection) = if let Ok(c) = camera_query.single() {
        c
    } else {
        cursor_pos.0 = None;
        return;
    };

    let window = if let Ok(w) = window_query.single() {
        w
    } else {
        cursor_pos.0 = None;
        return;
    };

    // prefer the engine conversion; fall back to a simple orthographic mapping
    if let Some(cursor_screen) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_screen) {
            cursor_pos.0 = Some(ray.origin.truncate());
            return;
        }

        if let Projection::Orthographic(ortho) = projection {
            let wnd = Vec2::new(window.width() as f32, window.height() as f32);
            let half = wnd * 0.5;
            // screen origin is bottom-left; center it then scale by ortho
            let screen_centered = (cursor_screen - half) * ortho.scale as f32;
            let world = camera_transform.translation().truncate() + screen_centered;
            cursor_pos.0 = Some(world);
            return;
        }
    }

    cursor_pos.0 = None;
}
