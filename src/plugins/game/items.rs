use bevy::prelude::*;

use crate::data::items::ItemData;
use crate::get_world_position_from_cursor_position;
use crate::plugins::{camera::MainCamera, TiledMapObjectLayer, TiledMapTileLayer};
use crate::tilemap::{get_tile_position, TileMapQuery};

#[derive(Debug, Component, Deref)]
pub struct Item(pub ItemData);

#[derive(Debug, Event)]
pub struct ItemDragEvent(pub Option<Vec2>);

impl ItemDragEvent {
    pub fn new(window: &Window) -> Self {
        Self(window.cursor_position())
    }
}

#[derive(Debug, Event)]
pub struct ItemDropEvent(pub Option<Vec2>);

impl ItemDropEvent {
    pub fn new(window: &Window) -> Self {
        Self(window.cursor_position())
    }
}

pub(super) fn item_drag_event_handler(
    mut events: EventReader<ItemDragEvent>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    object_layer_query: Query<TileMapQuery, With<TiledMapObjectLayer>>,
    tilemap_layer_query: Query<TileMapQuery, With<TiledMapTileLayer>>,
) {
    if events.is_empty() {
        return;
    }

    let (camera, camera_transform) = camera_query.single();

    // TODO: should we just deal with the first (or last?) event?
    // what does it even mean to have more than one of these ...
    for event in events.read() {
        let world_position =
            get_world_position_from_cursor_position(event.0, camera, camera_transform);
        if let Some(world_position) = world_position {
            // first check for objects
            let object_tilemap = object_layer_query.single();
            if let Some(object_position) = get_tile_position(
                world_position,
                object_tilemap.size,
                object_tilemap.grid_size,
                object_tilemap.r#type,
                object_tilemap.transform,
            ) {
                if let Some(_object_entity) = object_tilemap.storage.get(&object_position) {
                    info!("object at {:?}", object_position);
                    continue;
                }
            }

            // then check for tiles
            let tilemap = tilemap_layer_query.single();
            if let Some(tile_position) = get_tile_position(
                world_position,
                tilemap.size,
                tilemap.grid_size,
                tilemap.r#type,
                tilemap.transform,
            ) {
                if let Some(_tile_entity) = tilemap.storage.get(&tile_position) {
                    info!("tile at {:?}", tile_position);
                    continue;
                }
            }
        }
    }
}

pub(super) fn item_drop_event_handler(
    mut events: EventReader<ItemDropEvent>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if events.is_empty() {
        return;
    }

    let (camera, camera_transform) = camera_query.single();

    for event in events.read() {
        let world_position =
            get_world_position_from_cursor_position(event.0, camera, camera_transform);
        if let Some(world_position) = world_position {
            info!("ItemDropEvent: {:?}", world_position);

            // TODO: find the underlying tile and do something
        }
    }
}
