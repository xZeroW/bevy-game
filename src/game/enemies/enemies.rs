use std::f32::consts::PI;
use std::time::Duration;

use bevy::math::vec3;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::game::animation::animation::AnimationTimer;
use crate::game::player::{component::Player, atlas_index::AtlasIndex};
use crate::game::common::components::characters::position::Position;
use crate::game::game_state::GameState;
use crate::game::resources::GlobalTextureAtlas;
use crate::game::config as cfg;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

#[derive(Component, Default)]
pub struct TrackedEnemy;

#[derive(Component)]
pub enum EnemyType {
    Green,
    Red,
    Skin,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(cfg::ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                despawn_dead_enemies,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Enemy, Entity), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            println!("Enemy defeated!");
            commands.entity(entity).despawn();
        }
    }
}

fn update_enemy_transform(
    time: Res<Time>,
    player_query: Query<&Position, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut Sprite), (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos_comp = if let Ok(p) = player_query.single() { p } else { return };
    let dt = time.delta().as_secs_f32();

    for (mut transform, mut sprite) in enemy_query.iter_mut() {
        let enemy_pos2 = transform.translation.truncate();
        let player_pos2 = Vec2::new(player_pos_comp.x, player_pos_comp.y);
        let mut dir2 = player_pos2 - enemy_pos2;
        let len = dir2.length();
        if len <= std::f32::EPSILON {
            continue;
        }
        dir2 /= len;
        transform.translation += vec3(dir2.x, dir2.y, 0.0) * (cfg::ENEMY_SPEED * dt);

        // flip sprite to face player horizontally
        if player_pos2.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (cfg::MAX_NUM_ENEMIES - num_enemies).min(cfg::SPAWN_RATE_PER_SECOND);

    if num_enemies >= cfg::MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let player_tf = if let Ok(t) = player_query.single() { t } else { return };
    let player_pos = player_tf.translation.truncate();
    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_position_around(player_pos);
        let enemy_type = EnemyType::get_rand_enemy();
        commands.spawn((
            Sprite::from_atlas_image(
                handle.image.clone(),
                TextureAtlas {
                    layout: handle.layout.clone(),
                    index: enemy_type.get_base_sprite_index(),
                },
            ),
            Transform::from_translation(vec3(x, y, 1.0)).with_scale(Vec3::splat(cfg::SPRITE_SCALE as f32)),
            Enemy::default(),
            TrackedEnemy::default(),
            AtlasIndex(0),
            enemy_type,
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        ))
        ;
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..PI * 2.0);
    let dist = rng.random_range(1000.0..1500.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: cfg::ENEMY_HEALTH,
        }
    }
}

impl EnemyType {
    fn get_rand_enemy() -> Self {
        let mut rng = rand::rng();
        let rand_index = rng.random_range(0..3);
        return match rand_index {
            0 => Self::Green,
            1 => Self::Red,
            _ => Self::Skin,
        };
    }

    pub fn get_base_sprite_index(&self) -> usize {
        match self {
            EnemyType::Green => 8,
            EnemyType::Red => 12,
            EnemyType::Skin => 20,
        }
    }
}
