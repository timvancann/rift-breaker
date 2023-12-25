use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

#[derive(Resource, Default)]
pub struct Score(pub i32);

#[derive(Resource, Default)]
pub struct XP(pub f32);

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    GameOver,
    InGame,
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum InGameState {
    #[default]
    Running,
    Paused,
}
