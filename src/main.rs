mod components;
mod enemy;
mod map;
mod player;
mod rift;
mod systems;
mod ui;
mod resources;

use bevy::prelude::*;
use components::{MainCamera, MouseWorldCoords};
use enemy::EnemyPlugin;
use map::MapPlugin;
use player::{Player, PlayerPlugin};
use rift::RiftPlugin;
use systems::{cursor_world_position, handle_knockback, move_all};
use ui::{Score, UIPlugin};
use crate::resources::XP;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .init_resource::<MouseWorldCoords>()
        .init_resource::<Score>()
        .init_resource::<XP>()
        .add_plugins((PlayerPlugin, EnemyPlugin, UIPlugin, RiftPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc, cursor_world_position))
        .add_systems(FixedUpdate, (move_all, handle_knockback))
        .add_systems(PostUpdate, camer_follow_player)
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn camer_follow_player(
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let mut camera_transform = q_camera.single_mut();
    camera_transform.translation = q_player.single().translation;
}
