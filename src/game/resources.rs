use bevy::prelude::*;
use crate::game::config as cfg;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .add_systems(Startup, load_assets);
    }
}

pub fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
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
}
