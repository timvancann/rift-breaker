use bevy::prelude::Event;

#[derive(Event)]
pub struct PlayerHealthChanged {
    pub current: f32,
    pub max: f32,
}

#[derive(Event)]
pub struct PlayerDies;
