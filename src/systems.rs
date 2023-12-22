use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{Health, Knockback, MainCamera, MouseWorldCoords, Velocity};

pub fn cursor_world_position(
    mut coords: ResMut<MouseWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;
    }
}

pub fn move_all(mut q_movable: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in q_movable.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

pub fn die(mut commands: Commands, q_enemy: Query<(Entity, &Health)>) {
    for (entity, health) in q_enemy.iter() {
        if health.current <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_knockback(
    mut commands: Commands,
    mut q_enemy: Query<(&mut Transform, &Knockback, Entity)>,
) {
    for (mut transform, knockback, entity) in q_enemy.iter_mut() {
        if knockback.velocity != Vec2::ZERO {
            transform.translation.x += knockback.velocity.x;
            transform.translation.y += knockback.velocity.y;
            if (knockback.start_position - transform.translation.truncate()).length()
                >= knockback.distance
            {
                commands.entity(entity).remove::<Knockback>();
            }
        }
    }
}
