use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::assets::tiled::TiledMap;
use crate::components::MainCamera;

const CAMERA_SPEED: f32 = 200.0;

pub fn pan_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    tiledmaps: Res<Assets<TiledMap>>,
    mut _mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
    tiledmap_query: Query<&Handle<TiledMap>>,
) {
    let mut camera = camera_query.single_mut();
    let tiledmap = tiledmaps.get(tiledmap_query.single()).unwrap();

    let speed = CAMERA_SPEED * time.delta_seconds();

    if keys.pressed(KeyCode::ArrowRight) {
        camera.0.translation.x += speed;
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        camera.0.translation.x -= speed;
    }

    if keys.pressed(KeyCode::ArrowUp) {
        camera.0.translation.y += speed;
    }

    if keys.pressed(KeyCode::ArrowDown) {
        camera.0.translation.y -= speed;
    }

    info!(
        "camera view ({}, {}) to ({}, {})",
        camera.0.translation.x - camera.1.area.width() / 2.0,
        camera.0.translation.y - camera.1.area.height() / 2.0,
        camera.0.translation.x + camera.1.area.width() / 2.0,
        camera.0.translation.y + camera.1.area.height() / 2.0,
    );

    // TODO: there is probably a better way to get this info
    // without looking at the TiledMap. possibly TilemapSize and TilemapGridSize ?
    //     tilemap_query: Query<&TilemapSize>,
    info!(
        "map constraints: ({}, {}) to ({}, {})",
        -(((tiledmap.map.width * tiledmap.map.tile_width) / 2) as i32),
        -(((tiledmap.map.height * tiledmap.map.tile_height) / 2) as i32),
        ((tiledmap.map.width * tiledmap.map.tile_width) / 2),
        ((tiledmap.map.height * tiledmap.map.tile_height) / 2),
    );

    // TODO: need to constrain this to just the visible area
}
