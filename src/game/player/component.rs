use bevy::prelude::{Component, Sprite};

use crate::game::common::components::characters::move_speed::MoveSpeed;
use crate::game::common::components::characters::position::Position;
use crate::game::common::components::characters::char_state::State;

#[derive(Component, Default)]
#[require(Position, MoveSpeed, State, Sprite)]
pub struct Player;
