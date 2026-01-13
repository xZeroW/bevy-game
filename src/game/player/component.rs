use bevy::prelude::{Component, Bundle, Sprite};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub position: Position,
    pub move_speed: MoveSpeed,
    pub state: PlayerState,
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct MoveSpeed(pub u32);

#[derive(Component)]
pub enum PlayerState {
    Idle,
    Moving,
}
