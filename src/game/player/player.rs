use bevy::prelude::*;
use bevy::time::Stopwatch;
use crate::game::player::weapon::{Gun, GunTimer};

use crate::game::player::weapon::GunPlugin;
use crate::game::resources::GlobalTextureAtlas;
use crate::game::player::{
        animation::{PlayerAnimationPlugin, AnimationTimer},
        component::Player,
        atlas_index::AtlasIndex,
        controls::{controls, sync_position_transform, close_on_esc},
    };

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((PlayerAnimationPlugin, GunPlugin))
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
        ))
        .insert(AtlasIndex(2))
        .insert(AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)));

    // spawn the player's gun as a separate entity so systems can query it
    commands.spawn((
        Sprite::from_atlas_image(
            handle.image.clone(),
            TextureAtlas {
                layout: handle.layout.clone(),
                index: 17,
            },
        ),
        Gun,
        GunTimer(Stopwatch::new()),
        Transform::from_xyz(0.0, 0.0, 15.0),
    ));
}
