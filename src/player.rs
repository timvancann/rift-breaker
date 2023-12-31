use std::time::Duration;

use bevy::{math::vec3, prelude::*, sprite::collide_aabb::collide};

use crate::components::XpGem;
use crate::events::{PlayerDies, PlayerHealthChanged};
use crate::resources::AppState::InGame;
use crate::resources::XP;
use crate::{
    components::{Collider, Health, MouseWorldCoords, Movable, Velocity},
    enemy::Enemy,
};

const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const PLAYER_COLOR: Color = Color::YELLOW_GREEN;
const PLAYER_SPEED: f32 = 300.0;

const MAIN_WEAPON_SIZE: Vec2 = Vec2::new(30.0, 10.0);
const MAIN_WEAPON_COLOR: Color = Color::rgb(0.7, 0.3, 0.7);
const MAIN_WEAPON_OFFSET: f32 = 2.;
const MAIN_WEAPON_POSITION: f32 = PLAYER_SIZE.x + MAIN_WEAPON_OFFSET;

const WEAPON_NOZZLE_SIZE: Vec2 = Vec2::new(5.0, 5.0);
const WEAPON_NOZZLE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const WEAPON_NOZZLE_POSITION: f32 = MAIN_WEAPON_SIZE.x / 2. + 5.;
const WEAPON_RANGE: f32 = 700.0;

const BULLET_SPEED: f32 = 500.0;
const BULLET_SIZE: Vec2 = Vec2::new(5.0, 5.0);

const PICKUP_RADIUS: f32 = 75.0;

#[derive(Component)]
pub struct Player;

enum WeaponFireType {
    Primary,
    Secondary,
    Passive,
}

#[derive(Component)]
struct Weapon {
    cooldown_timer: Timer,
    weapon_fire_type: WeaponFireType,
}

#[derive(Component)]
struct Nozzle;

#[derive(Component)]
pub struct Bullet {
    spawn_location: Vec2,
}

#[derive(Component)]
struct RotatableAroundPlayer {
    offset: f32,
}

#[derive(Component)]
struct Invulnerable {
    timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGame), setup_player)
            .add_systems(
                FixedUpdate,
                (rotate_around_player, enemy_hits_player).run_if(in_state(InGame)),
            )
            .add_systems(
                Update,
                (
                    player_input,
                    fire_main,
                    despawn_bullets,
                    countdown_invulnerability,
                    pickup_xp_gem,
                    tick_weapon_cooldown,
                )
                    .run_if(in_state(InGame)),
            );
    }
}

fn setup_player(mut commands: Commands, mut ev_player_health: EventWriter<PlayerHealthChanged>) {
    let initial_player_health = 10.;
    let player = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    color: PLAYER_COLOR,
                    ..default()
                },
                ..default()
            },
            Player,
            Health {
                current: initial_player_health,
                max: initial_player_health,
            },
            Collider(PLAYER_SIZE),
            Velocity(Vec2::ZERO),
            Movable {
                move_speed: PLAYER_SPEED,
            },
        ))
        .id();

    ev_player_health.send(PlayerHealthChanged {
        current: initial_player_health,
        max: initial_player_health,
    });

    let main_weapon = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(MAIN_WEAPON_POSITION, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(MAIN_WEAPON_SIZE),
                    color: MAIN_WEAPON_COLOR,
                    ..default()
                },
                ..default()
            },
            RotatableAroundPlayer {
                offset: MAIN_WEAPON_POSITION,
            },
            Weapon {
                cooldown_timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
                weapon_fire_type: WeaponFireType::Primary,
            },
        ))
        .id();

    let nozzle = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(WEAPON_NOZZLE_POSITION, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(WEAPON_NOZZLE_SIZE),
                    color: WEAPON_NOZZLE_COLOR,
                    ..default()
                },
                ..default()
            },
            Nozzle,
        ))
        .id();

    commands.entity(player).push_children(&[main_weapon]);
    commands.entity(main_weapon).push_children(&[nozzle]);
}

fn player_input(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut velocity = query.single_mut();

    let mut vector = Vec2::ZERO;
    if input.pressed(KeyCode::A) {
        vector.x -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        vector.x += 1.0;
    }
    if input.pressed(KeyCode::W) {
        vector.y += 1.0;
    }
    if input.pressed(KeyCode::S) {
        vector.y -= 1.0;
    }

    if vector == Vec2::ZERO {
        velocity.x = 0.;
        velocity.y = 0.;
    } else {
        let displacement = vector.normalize() * PLAYER_SPEED;
        velocity.x = displacement.x;
        velocity.y = displacement.y;
    }
}

fn player_position(q_player: &Query<&GlobalTransform, With<Player>>) -> Vec2 {
    q_player.single().translation().truncate()
}

fn direction_to_mouse(
    mouse_world_coords: &Res<MouseWorldCoords>,
    q_player: &Query<&GlobalTransform, With<Player>>,
) -> Vec2 {
    let player_position = player_position(q_player);
    (mouse_world_coords.0 - player_position).normalize()
}

fn rotate_around_player(
    mouse_world_coords: Res<MouseWorldCoords>,
    mut q_rotatables: Query<(&mut Transform, &RotatableAroundPlayer)>,
    q_player: Query<&GlobalTransform, With<Player>>,
) {
    for (mut transform, rotatable) in q_rotatables.iter_mut() {
        let player_position = player_position(&q_player);
        if (mouse_world_coords.0 - player_position).length() < 10. {
            continue;
        }

        let direction = direction_to_mouse(&mouse_world_coords, &q_player);
        let angle = direction.y.atan2(direction.x);
        let position_on_circle = Vec2::new(angle.cos(), angle.sin()) * rotatable.offset;
        transform.translation = vec3(position_on_circle.x, position_on_circle.y, 0.0);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn fire_main(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    q_nozzle: Query<&Transform, With<Nozzle>>,
    mut q_weapon: Query<(&Transform, &mut Weapon)>,
    q_player: Query<&GlobalTransform, With<Player>>,
) {
    for (transform, mut weapon) in q_weapon.iter_mut() {
        if !(weapon.cooldown_timer.finished() || weapon.cooldown_timer.paused()) {
            continue;
        }

        if !input.just_pressed(MouseButton::Left) {
            continue;
        }

        let nozzle = q_nozzle.single();
        let player_position = player_position(&q_player);
        let direction = transform.translation.truncate().normalize();
        let nozzle_position = transform.rotation.mul_vec3(nozzle.translation).truncate()
            + player_position
            + transform.translation.truncate();
        let velocity = direction * BULLET_SPEED;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.2),
                    custom_size: Some(BULLET_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(nozzle_position.x, nozzle_position.y, 0.0),
                ..default()
            },
            Velocity(velocity),
            Bullet {
                spawn_location: nozzle_position,
            },
            Collider(BULLET_SIZE),
        ));

        weapon.cooldown_timer.reset();
    }
}

fn despawn_bullets(mut commands: Commands, q_bullet: Query<(&Transform, Entity, &Bullet)>) {
    for (transform, entity, bullet) in q_bullet.iter() {
        if (bullet.spawn_location - transform.translation.truncate()).length() > WEAPON_RANGE {
            commands.entity(entity).despawn();
        }
    }
}

fn countdown_invulnerability(
    mut commands: Commands,
    mut q_player: Query<(&mut Invulnerable, Entity)>,
    time: Res<Time>,
) {
    for (mut invulnerable, entity) in q_player.iter_mut() {
        invulnerable.timer.tick(time.delta());
        if invulnerable.timer.finished() {
            commands.entity(entity).remove::<Invulnerable>();
        }
    }
}

fn enemy_hits_player(
    mut commands: Commands,
    mut q_player: Query<
        (&Transform, &Collider, &mut Health, Entity),
        (With<Player>, Without<Invulnerable>),
    >,
    q_enemy: Query<(&Transform, &Collider), With<Enemy>>,
    mut ev_player_health: EventWriter<PlayerHealthChanged>,
    mut ev_player_dies: EventWriter<PlayerDies>,
) {
    for (player_transform, player_collider, mut player_health, entity) in q_player.iter_mut() {
        for (enemy_transform, enemy_collider) in q_enemy.iter() {
            if let Some(_) = collide(
                player_transform.translation,
                player_collider.0,
                enemy_transform.translation,
                enemy_collider.0,
            ) {
                player_health.current -= 1.;
                ev_player_health.send(PlayerHealthChanged {
                    current: player_health.current,
                    max: player_health.max,
                });
                commands.entity(entity).insert(Invulnerable {
                    timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
                });
                if player_health.current <= 0. {
                    ev_player_dies.send(PlayerDies);
                }

                return;
            }
        }
    }
}

fn tick_weapon_cooldown(mut q_weapon: Query<&mut Weapon>,
                        time: Res<Time>) {
    for mut weapon in q_weapon.iter_mut() {
        weapon.cooldown_timer.tick(time.delta());
    }
}

fn pickup_xp_gem(
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    q_xp_gem: Query<(&Transform, Entity, &XpGem)>,
    mut xp: ResMut<XP>,
) {
    let player_position = q_player.single().translation.truncate();
    for (transform, entity, gem) in q_xp_gem.iter() {
        if (player_position - transform.translation.truncate()).length() < PICKUP_RADIUS {
            commands.entity(entity).despawn();
            xp.0 += gem.0;
        }
    }
}
