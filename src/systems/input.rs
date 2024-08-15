use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::MainCamera;

pub fn tile_info(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        info!("tile info at {}", world_position);
        // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/mouse_to_tile.rs
    }
}

pub fn start_drag(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        info!("start drag at {}", world_position);
        // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/mouse_to_tile.rs
    }
}

pub fn stop_drag(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        info!("stop drag at {}", world_position);
        // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/mouse_to_tile.rs
    }
}

pub fn drag(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        info!("drag at {}", world_position);
        // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/mouse_to_tile.rs
    }
}
