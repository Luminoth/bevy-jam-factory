use bevy::{ecs::query::QueryData, prelude::*};

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
