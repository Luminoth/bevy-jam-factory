use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{
    camera::{CameraTransformQuery, MainCamera},
    tiled::TiledMapObjectLayer,
    tilemap::TileMapQuery,
};
use crate::get_world_position_from_cursor_position;
use crate::resources::game::{ObjectInfo, TileDrag};
use crate::tilemap::get_tile_position;

pub fn tile_info(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    object_layer_query: Query<TileMapQuery, With<TiledMapObjectLayer>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) =
        get_world_position_from_cursor_position(window, camera, camera_transform)
    {
        let tilemap = object_layer_query.single();
        if let Some(tile_position) = get_tile_position(
            world_position,
            tilemap.size,
            tilemap.grid_size,
            tilemap.r#type,
            tilemap.transform,
        ) {
            if let Some(tile_entity) = tilemap.storage.get(&tile_position) {
                commands.insert_resource(ObjectInfo(tile_entity));
            }
        }
    }
}

pub fn start_drag(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<CameraTransformQuery, With<MainCamera>>,
) {
    let camera = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) =
        get_world_position_from_cursor_position(window, camera.camera, camera.global_transform)
    {
        info!("start drag at {}", world_position);

        // TODO: figure out what entity to add here (there are multiple tilemap layers)
        commands.insert_resource(TileDrag::new(Entity::PLACEHOLDER));
    }
}

pub fn stop_drag(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<CameraTransformQuery, With<MainCamera>>,
) {
    let camera = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) =
        get_world_position_from_cursor_position(window, camera.camera, camera.global_transform)
    {
        info!("stop drag at {}", world_position);

        commands.remove_resource::<TileDrag>();
    }
}

pub fn drag(
    tile_drag: Option<ResMut<TileDrag>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Some(mut tile_drag) = tile_drag else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) =
        get_world_position_from_cursor_position(window, camera, camera_transform)
    {
        info!("drag at {}", world_position);

        // TODO: figure out what entity to add here (there are multiple tilemap layers)
        tile_drag.tiles.insert(Entity::PLACEHOLDER);
    }
}
