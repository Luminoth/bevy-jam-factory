use bevy::prelude::*;

use crate::data::items::ItemData;
use crate::get_world_position_from_cursor_position;
use crate::plugins::camera::MainCamera;

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
) {
    if events.is_empty() {
        return;
    }

    let (camera, camera_transform) = camera_query.single();

    for event in events.read() {
        let world_position =
            get_world_position_from_cursor_position(event.0, camera, camera_transform);
        if let Some(world_position) = world_position {
            info!("ItemDragEvent: {:?}", world_position);

            // TODO: find the underlying tile and color it
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
