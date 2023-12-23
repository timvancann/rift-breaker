mod components;
mod enemy;
mod player;
mod rift;
mod systems;
mod ui;

use bevy::prelude::*;
use components::{MainCamera, MouseWorldCoords};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use rift::RiftPlugin;
use systems::{cursor_world_position, handle_knockback, move_all};
use ui::{Score, UIPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .init_resource::<MouseWorldCoords>()
        .init_resource::<Score>()
        .add_plugins((PlayerPlugin, EnemyPlugin, UIPlugin, RiftPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc, cursor_world_position))
        .add_systems(FixedUpdate, (move_all, handle_knockback))
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
