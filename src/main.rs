mod components;
mod enemy;
mod player;
mod systems;

use bevy::prelude::*;
use components::{MainCamera, MouseWorldCoords};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use systems::{cursor_world_position, die, handle_knockback, move_all};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .init_resource::<MouseWorldCoords>()
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (bevy::window::close_on_esc, cursor_world_position, die),
        )
        .add_systems(FixedUpdate, (move_all, handle_knockback))
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
