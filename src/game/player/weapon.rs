use std::f32::consts::PI;
use bevy::platform::time::Instant;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

use crate::game::player::component::Player;
use crate::game::game_state::GameState;
use crate::game::config::{BULLET_SPEED, BULLET_SPAWN_INTERVAL, BULLET_TIME_SECS, NUM_BULLETS_PER_SHOT, SPRITE_SCALE};
use crate::game::resources::{CursorPosition, GlobalTextureAtlas};

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct GunTimer(pub Stopwatch);
#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct SpawnInstant(Instant);
#[derive(Component)]
struct BulletDirection(Vec3);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_gun_transform,
                update_bullets,
                handle_gun_input,
                despawn_old_bullets,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
) {
    for (instant, e) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > BULLET_TIME_SECS {
            commands.entity(e).despawn();
        }
    }
}

fn update_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }
    let player_pos = match player_query.single() {
        Ok(t) => t.translation.truncate(),
        Err(_) => return,
    };
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let mut gun_transform = match gun_query.single_mut() {
        Ok(g) => g,
        Err(_) => return,
    };

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos(),
        player_pos.y + offset * angle.sin(),
    );

    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
    gun_transform.translation.z = 15.0;
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (gun_transform, mut gun_timer) = match gun_query.single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let mut rng = rand::rng();
    let bullet_direction = gun_transform.local_x();
    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();

        for _ in 0..NUM_BULLETS_PER_SHOT {
            let dir = vec3(
                bullet_direction.x + rng.random_range(-0.5..0.5),
                bullet_direction.y + rng.random_range(-0.5..0.5),
                bullet_direction.z,
            );
            commands.spawn((
                Sprite::from_atlas_image(
                    handle.image.clone(),
                    TextureAtlas {
                        layout: handle.layout.clone(),
                        index: 16,
                    },
                ),
                Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE as f32)),
                Bullet,
                BulletDirection(dir),
                SpawnInstant(Instant::now()),
            ));
        }
    }
}

fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>,
    time: Res<Time>,
) {
    if bullet_query.is_empty() {
        return;
    }

    let delta = time.delta().as_secs_f32();
    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * BULLET_SPEED * delta;
        t.translation.z = 10.0;
    }
}
