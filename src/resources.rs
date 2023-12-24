use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

#[derive(Resource, Default)]
pub struct Score(pub i32);

#[derive(Resource, Default)]
pub struct XP(pub f32);