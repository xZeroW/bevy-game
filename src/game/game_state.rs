use bevy::prelude::*;

#[derive(Debug, Reflect, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Initializing,
    Loading,
    MainMenu,
    InGame,
    Paused,
    GameOver,
}
