use bevy::prelude::{Component, Sprite};

use crate::game::common::components::characters::{
    move_speed::MoveSpeed,
    health::Health,
    position::Position,
    char_state::State
};

#[derive(Component, Default, Debug)]
#[require(Position, Health, MoveSpeed, State, Sprite)]
pub struct Player;
