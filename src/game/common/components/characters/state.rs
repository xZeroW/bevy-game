use bevy::prelude::Component;

#[derive(Component, Default)]
pub enum State {
    #[default]
    Idle,
    Moving,
}
