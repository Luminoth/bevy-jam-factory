use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;

use crate::components::{
    objects::Object,
    tiled::{TiledMapObjectLayer, TiledMapTileLayer},
    MainCamera,
};
use crate::get_world_position_from_cursor_position;
use crate::resources::game::TileDrag;
use crate::tilemap::get_tile_position;

pub fn tile_info(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    object_layer_query: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
        ),
        With<TiledMapObjectLayer>,
    >,
    object_query: Query<&Object>,
    tile_layer_query: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
        ),
        With<TiledMapTileLayer>,
    >,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) =
        get_world_position_from_cursor_position(window, camera, camera_transform)
    {
        for (map_size, grid_size, map_type, tile_storage, map_transform) in
            object_layer_query.iter()
        {
            if let Some(tile_position) =
                get_tile_position(world_position, map_size, grid_size, map_type, map_transform)
            {
                if let Some(tile_entity) = tile_storage.get(&tile_position) {
                    let object = object_query
                        .get(tile_entity)
                        .expect("Object tile missing Object!");

                    info!("Got object: {:?}", object);
                    return;
                }
            }
        }

        for (map_size, grid_size, map_type, tile_storage, map_transform) in tile_layer_query.iter()
        {
            if let Some(tile_position) =
                get_tile_position(world_position, map_size, grid_size, map_type, map_transform)
            {
                if let Some(_tile_entity) = tile_storage.get(&tile_position) {
                    info!("Got tile");
                    return;
                }
            }
        }
    }
}

pub fn start_drag(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) =
        get_world_position_from_cursor_position(window, camera, camera_transform)
    {
        info!("start drag at {}", world_position);

        // TODO: figure out what entity to add here (there are multiple tilemap layers)
        commands.insert_resource(TileDrag::new(Entity::PLACEHOLDER));
    }
}

pub fn stop_drag(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) =
        get_world_position_from_cursor_position(window, camera, camera_transform)
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
