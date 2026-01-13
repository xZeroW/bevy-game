use bevy::prelude::Component;

#[derive(Component)]
pub enum State {
    Idle,
    Moving,
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}
