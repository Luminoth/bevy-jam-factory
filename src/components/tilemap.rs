use bevy::{ecs::query::QueryData, prelude::*};
use bevy_ecs_tilemap::prelude::*;

#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct TileMapSizeQuery {
    pub size: &'static TilemapSize,
    pub grid_size: &'static TilemapGridSize,
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct TileMapQuery {
    pub size: &'static TilemapSize,
    pub grid_size: &'static TilemapGridSize,
    pub r#type: &'static TilemapType,
    pub storage: &'static TileStorage,
    pub transform: &'static Transform,
}
