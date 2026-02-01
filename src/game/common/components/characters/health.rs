use bevy::prelude::*;

use crate::game::player::component::Player;
use crate::game::player::weapon::Weapon;
use crate::game::game_state::GameState;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            current: 100.0,
            max: 100.0,
        }
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Health { current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn take_damage(&mut self, damage: f32) {
        if self.current <= 0.0 {
            return;
        }

        self.current -= damage;
        if self.current < 0.0 {
            self.current = 0.0;
        }
        info!("Entity took {} damage, current health: {}/{}", damage, self.current, self.max);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_dead_entities.run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Player>)>,
    weapon_query: Query<Entity, With<Weapon>>,
) {
    // Loop por todas entidades com componente Health
    for (entity, health, maybe_player) in query.iter() {
        // se health <= 0
        if health.is_dead() {
            info!("Despawning entity {:?} with zero health", entity);
            // se for um player, despawna tbm suas armas
            if maybe_player.is_some() {
                for weapon_entity in weapon_query.iter() {
                    commands.entity(weapon_entity).despawn();
                }
            }

            commands.entity(entity).despawn();
        }
    }
}
