use bevy::prelude::*;
use crate::game::config as cfg;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .add_systems(Startup, load_sprite_sheet)
            .add_systems(Update, create_texture_atlas_layout.run_if(needs_atlas));
    }
}

fn needs_atlas(g: Res<GlobalTextureAtlas>) -> bool {
    g.layout.is_none() && g.image.is_some()
}

fn load_sprite_sheet(mut g: ResMut<GlobalTextureAtlas>, asset_server: Res<AssetServer>) {
    if g.image.is_none() {
        g.image = Some(asset_server.load("sprite-sheet.png"));
    }
}

fn create_texture_atlas_layout(
    mut g: ResMut<GlobalTextureAtlas>,
    images: Res<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if g.layout.is_some() {
        return;
    }

    let img_handle = match &g.image {
        Some(h) => h.clone(),
        None => return,
    };

    if let Some(img) = images.get(&img_handle) {
        let w = img.texture_descriptor.size.width;
        let h = img.texture_descriptor.size.height;
        let tile_w = cfg::SPRITE_PIX_W as u32;
        let tile_h = cfg::SPRITE_PIX_H as u32;
        let pad = cfg::SPRITE_PADDING as u32;

        if tile_w == 0 || tile_h == 0 {
            return;
        }

        let cols = (w / (tile_w + pad)).max(1);
        let rows = (h / (tile_h + pad)).max(1);

        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(tile_w, tile_h),
            cols,
            rows,
            Some(UVec2::splat(pad)),
            None,
        );

        g.layout = Some(layouts.add(layout));
    }
}
