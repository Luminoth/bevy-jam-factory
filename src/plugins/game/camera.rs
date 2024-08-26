use bevy::{ecs::query::QueryData, prelude::*, window::PrimaryWindow};

use crate::plugins::{IsPointerCaptured, TiledMapTileLayer};
use crate::tilemap::TileMapSizeQuery;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UiCamera;

#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct CameraTransformQuery {
    pub camera: &'static Camera,
    pub transform: &'static Transform,
    pub global_transform: &'static GlobalTransform,
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct CameraProjectionQuery {
    pub transform: &'static Transform,
    pub global_transform: &'static GlobalTransform,
    pub projection: &'static OrthographicProjection,
}

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct CameraProjectionQueryMut {
    pub transform: &'static mut Transform,
    pub global_transform: &'static GlobalTransform,
    pub projection: &'static OrthographicProjection,
}

const CAMERA_SPEED: f32 = 200.0;

pub(super) fn pan(
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

    let Ok(tilemap) = tilemap_query.get_single() else {
        return;
    };
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
