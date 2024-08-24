use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{camera::*, tilemap::TileMapSizeQuery};
use crate::plugins::{IsPointerCaptured, TiledMapTileLayer};

// TODO: this should either be a plugin or be part of the game plugin

const CAMERA_SPEED: f32 = 200.0;

pub fn pan(
    is_pointer_captured: Res<IsPointerCaptured>,
    _keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<CameraProjectionQueryMut, With<MainCamera>>,
    tilemap_query: Query<TileMapSizeQuery, With<TiledMapTileLayer>>,
) {
    if is_pointer_captured.0 {
        return;
    }

    let window = window_query.single();
    let window_movement_width = window.width() / 4.0;
    let window_movement_height = window.height() / 4.0;

    let mut camera = camera_query.single_mut();
    let view_half_width = camera.projection.area.width() / 2.0;
    let view_half_height = camera.projection.area.height() / 2.0;

    let tilemap = tilemap_query.single();
    let map_half_width = (tilemap.size.x as f32 * tilemap.grid_size.x) / 2.0;
    let map_half_height = (tilemap.size.y as f32 * tilemap.grid_size.y) / 2.0;

    let speed = CAMERA_SPEED * time.delta_seconds();

    if let Some(position) = window.cursor_position() {
        if position.x < window_movement_width {
            camera.transform.translation.x =
                (camera.transform.translation.x - speed).max(view_half_width - map_half_width);
        } else if position.x > window.width() - window_movement_width {
            camera.transform.translation.x =
                (camera.transform.translation.x + speed).min(map_half_width - view_half_width);
        }

        if position.y < window_movement_height {
            camera.transform.translation.y =
                (camera.transform.translation.y + speed).min(map_half_height - view_half_height);
        } else if position.y > window.height() - window_movement_height {
            camera.transform.translation.y =
                (camera.transform.translation.y - speed).max(view_half_height - map_half_height);
        }
    }

    // TODO: this could be useful for debugging
    /*if keys.pressed(KeyCode::ArrowRight) {
        camera.transform.translation.x =
            (camera.transform.translation.x + speed).min(map_half_width - view_half_width);
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        camera.transform.translation.x =
            (camera.transform.translation.x - speed).max(view_half_width - map_half_width);
    }

    if keys.pressed(KeyCode::ArrowUp) {
        camera.transform.translation.y =
            (camera.transform.translation.y + speed).min(map_half_height - view_half_height);
    }

    if keys.pressed(KeyCode::ArrowDown) {
        camera.transform.translation.y =
            (camera.transform.translation.y - speed).max(view_half_height - map_half_height);
    }*/
}
