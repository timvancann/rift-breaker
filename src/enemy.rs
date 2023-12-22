use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    components::{Collider, Health, Knockback, Velocity},
    player::Bullet,
};
const ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy)
            .add_systems(FixedUpdate, (bullet_hit_enemy, handle_knockback));
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
        Knockback {
            distance: 20.0,
            ..default()
        },
        Health {
            current: 10.,
            max: 10.,
        },
    ));
}

fn bullet_hit_enemy(
    mut commands: Commands,
    q_bullet: Query<(&Transform, Entity, &Collider, &Velocity), With<Bullet>>,
    mut q_enemy: Query<(&Transform, &Collider, &mut Health, &mut Knockback), With<Enemy>>,
    time: Res<Time<Fixed>>,
) {
    for (bullet_transform, bullet_entity, bullet_collider, velocity) in q_bullet.iter() {
        for (enemy_transform, enemy_collider, mut health, mut knockback) in q_enemy.iter_mut() {
            if let Some(_) = collide(
                bullet_transform.translation,
                bullet_collider.0,
                enemy_transform.translation,
                enemy_collider.0,
            ) {
                commands.entity(bullet_entity).despawn();
                health.current -= 1.;
                knockback.velocity = velocity.normalize() * 20. * time.delta().as_secs_f32();
                knockback.start_position = enemy_transform.translation.truncate();
                println!("enemy health: {}", health.current);
            }
        }
    }
}

fn handle_knockback(mut q_enemy: Query<(&mut Transform, &mut Knockback)>) {
    for (mut transform, mut knockback) in q_enemy.iter_mut() {
        if knockback.velocity != Vec2::ZERO {
            transform.translation.x += knockback.velocity.x;
            transform.translation.y += knockback.velocity.y;
            if (knockback.start_position - transform.translation.truncate()).length()
                >= knockback.distance
            {
                knockback.velocity = Vec2::ZERO;
            }
        }
    }
}
