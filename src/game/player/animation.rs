use bevy::prelude::*;

use crate::game::{
    common::components::characters::char_state::State, game_state::GameState, player::{atlas_index::AtlasIndex, component::Player}, resources::GlobalTextureAtlas
};

pub struct PlayerAnimationPlugin;

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animation_timer_tick, animate_player).run_if(in_state(GameState::InGame)),
        );
    }
}

fn animation_timer_tick(time: Res<Time>, mut query: Query<&mut AnimationTimer>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}

fn animate_player(
    mut player_query: Query<(&mut Sprite, &mut AtlasIndex, &State, &AnimationTimer), With<Player>>,
    atlas: Res<GlobalTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }
    if let Ok((mut sprite, mut aindex, state, timer)) = player_query.single_mut() {
        if timer.0.just_finished() {
            match state {
                State::Idle => {
                    aindex.0 = 2;
                    let new_index = 2;
                    sprite.clone_from(&Sprite::from_atlas_image(
                        atlas.image.clone(),
                        TextureAtlas {
                            layout: atlas.layout.clone(),
                            index: new_index,
                        },
                    ));
                }
                State::Moving => {
                    aindex.0 = (aindex.0 + 1) % 4;
                    let new_index = 4 + aindex.0;
                    sprite.clone_from(&Sprite::from_atlas_image(
                        atlas.image.clone(),
                        TextureAtlas {
                            layout: atlas.layout.clone(),
                            index: new_index,
                        },
                    ));
                }
            }
        }
    }
}

// fn flip_gun_sprite_y(
//     cursor_position: Res<CursorPosition>,
//     mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
// ) {
//     if gun_query.is_empty() {
//         return;
//     }

//     if let Ok((mut sprite, transform)) = gun_query.single_mut() {
//         if let Some(cursor_position) = cursor_position.0 {
//             if cursor_position.x > transform.translation.x {
//                 sprite.flip_y = false;
//             } else {
//                 sprite.flip_y = true;
//             }
//         }
//     }
// }
