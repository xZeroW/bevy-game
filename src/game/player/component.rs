use bevy::prelude::{Component, Sprite};

use crate::game::common::components::characters::{
    move_speed::MoveSpeed,
    health::Health,
    position::Position,
    char_state::State,
    stats::Stats,
};

#[derive(Component, Debug)]
#[require(Position, Health, MoveSpeed, State, Stats, Sprite)]
pub struct Player;
