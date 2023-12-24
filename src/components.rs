use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Collider(pub Vec2);

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Default)]
pub struct Movable {
    pub move_speed: f32,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Knockback {
    pub velocity: Vec2,
    pub start_position: Vec2,
    pub distance: f32,
}

#[derive(Component)]
pub struct XpGem(pub f32);
