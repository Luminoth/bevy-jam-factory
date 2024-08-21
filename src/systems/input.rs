use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{
    camera::{CameraTransformQuery, MainCamera},
    ui::IsPointerCaptured,
};
use crate::get_world_position_from_cursor_position;
use crate::resources::game::TileDrag;

pub fn start_drag(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<CameraTransformQuery, With<MainCamera>>,
    capture_query: Query<&IsPointerCaptured>,
) {
    if capture_query.single().0 {
        return;
    }

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
    tile_drag: Option<ResMut<TileDrag>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<CameraTransformQuery, With<MainCamera>>,
    //capture_query: Query<&IsPointerCaptured>,
) {
    if tile_drag.is_none() {
        return;
    }

    /*if capture_query.single().0 {
        return;
    }*/

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
    capture_query: Query<&IsPointerCaptured>,
) {
    let Some(mut tile_drag) = tile_drag else {
        return;
    };

    if capture_query.single().0 {
        return;
    }

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
