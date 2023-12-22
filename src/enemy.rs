use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    components::{Collider, Health, Knockback, Movable, Velocity},
    player::{Bullet, Player},
};
const ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy)
            .add_systems(Update, move_towards_player_when_not_knockback)
            .add_systems(FixedUpdate, bullet_hit_enemy);
    }
}

#[derive(Component)]
struct Enemy;

fn setup_enemy(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                custom_size: Some(ENEMY_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(200.0, 100.0, 0.0),
            ..default()
        },
        Collider(ENEMY_SIZE),
        Enemy,
        Health {
            current: 10.,
            max: 10.,
        },
        Velocity(Vec2::ZERO),
        Movable { move_speed: 100. },
    ));
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
