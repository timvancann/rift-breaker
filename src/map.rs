use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

use crate::{components::MainCamera, player::Player};

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 8,
    y: CHUNK_SIZE.y * 8,
};

#[derive(Component)]
pub struct MapPlugin;

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
            .insert_resource(ChunkManager::default())
            .add_systems(
                Update,
                (spawn_chunks_around_camera, despawn_outofrange_chunks),
            )
            .add_plugins(TilemapPlugin);
    }
}

#[derive(Component)]
struct Tile;

fn spawn_chunk(commands: &mut Commands, asset_server: &AssetServer, chunk_pos: IVec2) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    // Spawn the elements of the tilemap.
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        -10.0,
    ));
    let texture_handle: Handle<Image> = asset_server.load("textures/grass_tile.png");
    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size: TILE_SIZE.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: TILE_SIZE,
            transform,
            ..default()
        },
        Tile, )
    );
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for transform in q_camera.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &asset_server, IVec2::new(x, y));
                }
            }
        }
    }
}

fn despawn_outofrange_chunks(
    mut commands: Commands,
    q_camera: Query<&Transform, With<Camera>>,
    q_chunks: Query<(Entity, &Transform), With<Tile>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for camera_transform in q_camera.iter() {
        for (entity, chunk_transform) in q_chunks.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > 320.0 {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}
