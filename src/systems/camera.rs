use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;

use crate::components::tiled::TiledMapTileLayer;
use crate::components::MainCamera;

const CAMERA_SPEED: f32 = 200.0;

pub fn pan_camera(
    _keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
    tilemap_query: Query<(&TilemapSize, &TilemapGridSize), With<TiledMapTileLayer>>,
) {
    // this can happen if the tilemap isn't loaded yet
    // TODO: we shouldn't even be running this until the tilemap is loaded
    if tilemap_query.is_empty() {
        return;
    }

    let window = window_query.single();
    let window_movement_width = window.width() / 4.0;
    let window_movement_height = window.height() / 4.0;

    let mut camera = camera_query.single_mut();
    let view_half_width = camera.1.area.width() / 2.0;
    let view_half_height = camera.1.area.height() / 2.0;

    let (tilemap_size, tile_size) = tilemap_query.single();
    let map_half_width = (tilemap_size.x as f32 * tile_size.x) / 2.0;
    let map_half_height = (tilemap_size.y as f32 * tile_size.y) / 2.0;

    let speed = CAMERA_SPEED * time.delta_seconds();

    if let Some(position) = window.cursor_position() {
        if position.x < window_movement_width {
            camera.0.translation.x =
                (camera.0.translation.x - speed).max(view_half_width - map_half_width);
        } else if position.x > window.width() - window_movement_width {
            camera.0.translation.x =
                (camera.0.translation.x + speed).min(map_half_width - view_half_width);
        }

        if position.y < window_movement_height {
            camera.0.translation.y =
                (camera.0.translation.y + speed).min(map_half_height - view_half_height);
        } else if position.y > window.height() - window_movement_height {
            camera.0.translation.y =
                (camera.0.translation.y - speed).max(view_half_height - map_half_height);
        }
    }

    // TODO: this could be useful for debugging
    /*if keys.pressed(KeyCode::ArrowRight) {
        camera.0.translation.x =
            (camera.0.translation.x + speed).min(map_half_width - view_half_width);
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        camera.0.translation.x =
            (camera.0.translation.x - speed).max(view_half_width - map_half_width);
    }

    if keys.pressed(KeyCode::ArrowUp) {
        camera.0.translation.y =
            (camera.0.translation.y + speed).min(map_half_height - view_half_height);
    }

    if keys.pressed(KeyCode::ArrowDown) {
        camera.0.translation.y =
            (camera.0.translation.y - speed).max(view_half_height - map_half_height);
    }*/
}
