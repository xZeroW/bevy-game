use bevy::prelude::{Bundle, Sprite};

use crate::game::common::components::characters::move_speed::MoveSpeed;
use crate::game::common::components::characters::position::Position;
use crate::game::common::components::characters::state::State;

#[derive(Bundle)]
pub struct Player {
    pub sprite: Sprite,
    pub position: Position,
    pub move_speed: MoveSpeed,
    pub state: State,
}
