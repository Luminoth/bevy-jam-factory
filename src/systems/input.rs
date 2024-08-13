use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::MainCamera;

pub fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if buttons.just_released(MouseButton::Right) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            info!("right click at {}", world_position);
            // TODO: send event
            // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/mouse_to_tile.rs
        }
    }

    if buttons.pressed(MouseButton::Left) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            info!("left held at {}", world_position);
            // TODO: send event
            // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/mouse_to_tile.rs
        }
    }
}
