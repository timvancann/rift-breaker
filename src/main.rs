use bevy::{math::vec3, prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};

const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const PLAYER_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const PLAYER_SPEED: f32 = 300.0;

const MAIN_WEAPON_SIZE: Vec2 = Vec2::new(30.0, 10.0);
const MAIN_WEAPON_COLOR: Color = Color::rgb(0.7, 0.3, 0.7);
const MAIN_WEAPON_OFFSET: f32 = 10.;
const MAIN_WEAPON_POSITION: f32 = PLAYER_SIZE.x + MAIN_WEAPON_OFFSET;

const WEAPON_NOZZLE_SIZE: Vec2 = Vec2::new(5.0, 5.0);
const WEAPON_NOZZLE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const WEAPON_NOZZLE_OFFSET: f32 = 1.;
const WEAPON_NOZZLE_POSITION: f32 = MAIN_WEAPON_POSITION + WEAPON_NOZZLE_OFFSET;
const WEAPON_RANGE: f32 = 700.0;

const BULLET_SPEED: f32 = 500.0;
const BULLET_SIZE: Vec2 = Vec2::new(5.0, 5.0);

const ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .init_resource::<MouseWorldCoords>()
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                cursor_world_position,
                handle_knockback,
                die,
            ),
        )
        .run();
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(FixedUpdate, (rotate_around_player, move_all))
            .add_systems(
                Update,
                (move_player, fire, despawn_bullets, bullet_hit_enemy),
            );
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

#[derive(Component)]
struct Collider(Vec2);

#[derive(Component)]
struct MainWeapon;

#[derive(Component)]
struct Nozzle;

#[derive(Component)]
struct Bullet {
    spawn_location: Vec2,
}

#[derive(Resource, Default)]
struct MouseWorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct RotatableAroundPlayer {
    offset: f32,
}

fn setup_player(mut commands: Commands) {
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
            Velocity(Vec2::ZERO),
        ))
        .id();

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
            MainWeapon,
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

#[derive(Component)]
struct Enemy;

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

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

fn cursor_world_position(
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
fn move_all(mut q_movable: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in q_movable.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn move_player(
    input: Res<Input<KeyCode>>,
    time: Res<Time<Fixed>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
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
        let displacement = vector.normalize() * time.delta_seconds() * PLAYER_SPEED;
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
        if (mouse_world_coords.0 - player_position).length() < rotatable.offset {
            continue;
        }

        let direction = direction_to_mouse(&mouse_world_coords, &q_player);
        let angle = direction.y.atan2(direction.x);
        let position_on_circle = Vec2::new(angle.cos(), angle.sin()) * rotatable.offset;
        transform.translation = vec3(position_on_circle.x, position_on_circle.y, 0.0);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn fire(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    q_nozzle: Query<&Transform, With<Nozzle>>,
    q_weapon: Query<&Transform, With<MainWeapon>>,
    q_player: Query<&GlobalTransform, With<Player>>,
    time: Res<Time<Fixed>>,
) {
    if input.just_pressed(MouseButton::Left) {
        let nozzle = q_nozzle.single();
        let weapon = q_weapon.single();
        let player_position = player_position(&q_player);
        let direction = weapon.translation.truncate().normalize();
        let nozzle_position = weapon.rotation.mul_vec3(nozzle.translation).truncate()
            + player_position
            + weapon.translation.truncate();
        let velocity = direction * BULLET_SPEED * time.delta().as_secs_f32();

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
    }
}

fn despawn_bullets(mut commands: Commands, q_bullet: Query<(&Transform, Entity, &Bullet)>) {
    for (transform, entity, bullet) in q_bullet.iter() {
        if (bullet.spawn_location - transform.translation.truncate()).length() > WEAPON_RANGE {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Default)]
struct Knockback {
    velocity: Vec2,
    start_position: Vec2,
    distance: f32,
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

fn die(mut commands: Commands, q_enemy: Query<(Entity, &Health)>) {
    for (entity, health) in q_enemy.iter() {
        if health.current <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
