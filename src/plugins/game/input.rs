use bevy::{prelude::*, window::PrimaryWindow};

use super::camera::{CameraTransformQuery, MainCamera};
use super::TileDrag;
use crate::get_world_position_from_cursor_position;
use crate::plugins::ui::IsPointerCaptured;

pub(super) fn start_drag(
    mut commands: Commands,
    is_pointer_captured: Res<IsPointerCaptured>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<CameraTransformQuery, With<MainCamera>>,
) {
    if is_pointer_captured.0 {
        return;
    }

    let camera = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = get_world_position_from_cursor_position(
        window.cursor_position(),
        camera.camera,
        camera.global_transform,
    ) {
        info!("start drag at {}", world_position);

        // TODO: figure out what entity to add here (there are multiple tilemap layers)
        commands.insert_resource(TileDrag::new(Entity::PLACEHOLDER));
    }
}

pub(super) fn stop_drag(
    mut commands: Commands,
    //is_pointer_captured: Res<IsPointerCaptured>,
    tile_drag: Option<ResMut<TileDrag>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<CameraTransformQuery, With<MainCamera>>,
) {
    if tile_drag.is_none() {
        return;
    }

    /*if is_pointer_captured.0 {
        return;
    }*/

    let camera = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = get_world_position_from_cursor_position(
        window.cursor_position(),
        camera.camera,
        camera.global_transform,
    ) {
        info!("stop drag at {}", world_position);

        commands.remove_resource::<TileDrag>();
    }
}

pub(super) fn drag(
    is_pointer_captured: Res<IsPointerCaptured>,
    tile_drag: Option<ResMut<TileDrag>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Some(mut tile_drag) = tile_drag else {
        return;
    };

    if is_pointer_captured.0 {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) =
        get_world_position_from_cursor_position(window.cursor_position(), camera, camera_transform)
    {
        info!("drag at {}", world_position);

        // TODO: figure out what entity to add here (there are multiple tilemap layers)
        tile_drag.tiles.insert(Entity::PLACEHOLDER);
    }
}
