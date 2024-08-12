use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_ecs_tilemap::prelude::*;

use crate::components::tiled::TiledMapTileLayer;
use crate::components::MainCamera;

const CAMERA_SPEED: f32 = 200.0;

pub fn pan_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut _mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
    tilemap_query: Query<(&TilemapSize, &TilemapGridSize), With<TiledMapTileLayer>>,
) {
    if tilemap_query.is_empty() {
        return;
    }

    let mut camera = camera_query.single_mut();
    let view_half_width = camera.1.area.width() / 2.0;
    let view_half_height = camera.1.area.height() / 2.0;

    let (tilemap_size, tile_size) = tilemap_query.single();
    let map_half_width = (tilemap_size.x as f32 * tile_size.x) / 2.0;
    let map_half_height = (tilemap_size.y as f32 * tile_size.y) / 2.0;

    let speed = CAMERA_SPEED * time.delta_seconds();

    // TODO: use the mouse instead of the keyboard

    if keys.pressed(KeyCode::ArrowRight) {
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
    }
}
