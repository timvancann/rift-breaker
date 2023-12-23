use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    components::{Collider, Health, Knockback, Movable, Velocity},
    player::{Bullet, Player},
    ui::Score,
};
const ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const ENEMY_HEALTH: f32 = 2.;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_towards_player_when_not_knockback, die))
            .add_systems(FixedUpdate, bullet_hit_enemy);
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    sprite: SpriteBundle,
    collider: Collider,
    enemy: Enemy,
    health: Health,
    velocity: Velocity,
    movable: Movable,
}

pub fn prepare_enemy(location: &Vec2) -> EnemyBundle {
    EnemyBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                custom_size: Some(ENEMY_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(location.x, location.y, 0.),
            ..default()
        },
        collider: Collider(ENEMY_SIZE),
        enemy: Enemy,
        health: Health {
            current: ENEMY_HEALTH,
            max: ENEMY_HEALTH,
        },
        velocity: Velocity(Vec2::ZERO),
        movable: Movable { move_speed: 100. },
    }
}

fn bullet_hit_enemy(
    mut commands: Commands,
    q_bullet: Query<(&Transform, Entity, &Collider, &Velocity), With<Bullet>>,
    mut q_enemy: Query<(&Transform, &Collider, &mut Health, Entity), With<Enemy>>,
) {
    for (bullet_transform, bullet_entity, bullet_collider, velocity) in q_bullet.iter() {
        for (enemy_transform, enemy_collider, mut health, entity) in q_enemy.iter_mut() {
            if let Some(_) = collide(
                bullet_transform.translation,
                bullet_collider.0,
                enemy_transform.translation,
                enemy_collider.0,
            ) {
                commands.entity(bullet_entity).despawn();
                health.current -= 1.;
                let knockback = Knockback {
                    velocity: velocity.normalize() * 20.,
                    start_position: enemy_transform.translation.truncate(),
                    distance: 10.,
                };

                commands.entity(entity).insert(knockback);
            }
        }
    }
}

fn move_towards_player_when_not_knockback(
    mut q_enemy: Query<(&Transform, &mut Velocity, &Movable), (With<Enemy>, Without<Knockback>)>,
    q_player: Query<&Transform, With<Player>>,
) {
    for (enemy_transform, mut velocity, movable) in q_enemy.iter_mut() {
        let player_transform = q_player.single();
        let direction = (player_transform.translation.truncate()
            - enemy_transform.translation.truncate())
        .normalize();
        velocity.0 = direction * movable.move_speed;
    }
}

fn die(
    mut commands: Commands,
    q_enemy: Query<(Entity, &Health), With<Enemy>>,
    mut score: ResMut<Score>,
) {
    for (entity, health) in q_enemy.iter() {
        if health.current <= 0. {
            commands.entity(entity).despawn();
            score.0 += 1;
        }
    }
}
