use bevy::prelude::*;
use rand::prelude::*;
use rand_distr::{Distribution, UnitCircle};
use std::time::Duration;

use crate::resources::AppState::InGame;
use crate::{enemy::prepare_enemy, player::Player};

const RIFT_COLOR: Color = Color::PURPLE;
const RIFT_SIZE: Vec2 = Vec2::new(100.0, 100.0);
const RIFT_SPAWN_RADIUS: f32 = 500.0;

#[derive(Component)]
pub struct RiftPlugin;

impl Plugin for RiftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGame), setup_rift_spawning)
            .add_systems(
                Update,
                (spawn_rift, spawn_enemies, destroy_rift).run_if(in_state(InGame)),
            );
    }
}

#[derive(Component)]
struct Rift {
    enemies_to_spawn: u32,
}

#[derive(Resource)]
struct RiftSpawnConfig {
    timer: Timer,
}

#[derive(Component)]
struct EnemySpawnConfig {
    timer: Timer,
}

fn setup_rift_spawning(mut commands: Commands) {
    commands.insert_resource(RiftSpawnConfig {
        timer: Timer::new(Duration::from_secs(4), TimerMode::Repeating),
    })
}

fn random_point_on_unit_circle(radius: f32) -> Vec2 {
    let mut rng = thread_rng();
    let point = UnitCircle.sample(&mut rng);
    Vec2::new(point[0], point[1]) * radius
}

fn spawn_rift(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<RiftSpawnConfig>,
    q_player_transform: Query<&Transform, With<Player>>,
) {
    config.timer.tick(time.delta());
    let player_position = q_player_transform.single().translation.truncate();

    if config.timer.finished() {
        let random_spawn_point = player_position + random_point_on_unit_circle(RIFT_SPAWN_RADIUS);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: RIFT_COLOR,
                    custom_size: Some(RIFT_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(random_spawn_point.x, random_spawn_point.y, 0.0),
                ..default()
            },
            Rift {
                enemies_to_spawn: 5,
            },
            EnemySpawnConfig {
                timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
            },
        ));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut q_rift: Query<(&mut EnemySpawnConfig, &mut Rift, &Transform)>,
) {
    for (mut config, mut rift, transform) in q_rift.iter_mut() {
        config.timer.tick(time.delta());
        if config.timer.finished() {
            commands.spawn(prepare_enemy(&transform.translation.truncate()));
            rift.enemies_to_spawn -= 1;
        }
    }
}

fn destroy_rift(mut commands: Commands, q_rift: Query<(Entity, &Rift)>) {
    for (entity, rift) in q_rift.iter() {
        if rift.enemies_to_spawn <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
