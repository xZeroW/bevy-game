use bevy::prelude::*;
use crate::game::player::component::Player;
use crate::game::player::controls::{controls, sync_position_transform, close_on_esc};
use crate::game::resources::GlobalTextureAtlas;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup.after(crate::game::resources::load_assets))
        .add_systems(Update,
            (controls, close_on_esc))
        .add_systems(FixedUpdate, sync_position_transform);
    }
}

fn setup(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    commands.spawn(Player)
        .insert(Sprite::from_atlas_image(
            handle.image.clone(),
            TextureAtlas {
                layout:handle.layout.clone(),
                index:2
            }
        ));
}
