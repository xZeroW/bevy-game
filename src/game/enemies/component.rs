use bevy::prelude::Component;

use crate::game::common::components::characters::{
    health::Health,
};

#[derive(Component, Default)]
#[require(Health)]
pub struct Enemy;

impl Enemy {
    pub fn new() -> Self {
        Enemy
    }
}
