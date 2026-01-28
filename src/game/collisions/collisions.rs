use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::game::player::component::Player;
use crate::game::enemies::enemies::Enemy;
use crate::game::player::weapon::Bullet;
use crate::game::game_state::GameState;
use crate::game::config as cfg;
use crate::game::spatial::{KDTree2, Collidable};
use crate::game::enemies::enemies::TrackedEnemy;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KDTree2::default())
            .add_systems(
                Update,
                (
                    update_enemy_kd_tree
                        .run_if(on_timer(Duration::from_secs_f32(cfg::KD_TREE_REFRESH_RATE))),
                    handle_enemy_bullet_collision,
                    handle_enemy_player_collision,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn handle_enemy_player_collision(player_query: Query<&Transform, With<Player>>, tree: Res<KDTree2>) {
    if player_query.is_empty() {
        return;
    }
    let player_pos = match player_query.single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    if let Some((pos, _entity)) = tree.nearest_neighbour(player_pos) {
        if pos.distance(player_pos) <= 20.0 {
            info!("player-enemy collision detected");
        }
    }
}

// AutomaticUpdate plugin maintains the KDTree resource; no manual update needed.
fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    tree: Res<KDTree2>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }
    for (b_entity, b_t) in bullet_query.iter() {
        let bullet_pos = b_t.translation;
        if let Some((nearest_pos, entity)) = tree.nearest_neighbour(bullet_pos) {
            if nearest_pos.distance(bullet_pos) <= 20.0 {
                if let Some(e) = entity {
                    if let Ok((_, mut enemy)) = enemy_query.get_mut(e) {
                        enemy.health -= cfg::BULLET_DAMAGE;
                        // remove bullet so it doesn't hit again
                        commands.entity(b_entity).despawn();
                    }
                }
            }
        }
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<KDTree2>,
    enemy_query: Query<(&Transform, Entity), With<TrackedEnemy>>,
) {
    let mut items = Vec::new();
    for (t, e) in enemy_query.iter() {
        items.push(Collidable { pos: [t.translation.x, t.translation.y], entity: e });
    }

    tree.rebuild(items);
}

// no KdPoint impl or manual tree default needed with `bevy_spatial` AutomaticUpdate
