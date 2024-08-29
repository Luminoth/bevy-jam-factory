use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::data::items::{ItemData, ItemType};
use crate::get_world_position_from_cursor_position;
use crate::plugins::{
    camera::MainCamera, /*game_ui::inventory::hide_inventory_drag_icon,*/ objects::Object,
    Inventory, InventoryUpdatedEvent, TiledMapObjectLayer, TiledMapTileLayer,
};
use crate::tilemap::{get_tile_position, TileMapQuery};

#[derive(Debug, Component, Deref)]
pub struct Item(pub ItemData);

#[derive(Debug, Resource)]
pub struct ItemDragObject(pub Entity);

#[derive(Debug, Resource)]
pub struct ItemDragTile(pub Entity);

#[derive(Debug, Event)]
pub struct ItemDragEvent {
    pub item_type: ItemType,
    pub cursor_position: Option<Vec2>,
}

impl ItemDragEvent {
    pub fn new(window: &Window, item_type: ItemType) -> Self {
        Self {
            item_type,
            cursor_position: window.cursor_position(),
        }
    }
}

#[derive(Debug, Event)]
pub struct ItemDropEvent {
    pub item_type: ItemType,
    pub cursor_position: Option<Vec2>,
    pub drag_image_id: Entity,
    pub drag_image_position: (Val, Val),
}

impl ItemDropEvent {
    pub fn new(
        window: &Window,
        item_type: ItemType,
        drag_image_id: Entity,
        drag_image_style: &Style,
    ) -> Self {
        Self {
            item_type,
            cursor_position: window.cursor_position(),
            drag_image_id,
            drag_image_position: (drag_image_style.left, drag_image_style.top),
        }
    }
}

const CAN_DROP_COLOR: Color = Color::srgba(0.0, 1.0, 0.0, 0.5);
const NO_DROP_COLOR: Color = Color::srgba(1.0, 0.0, 0.0, 0.5);

// TODO: we might be able to simplify this by splitting it into
// an object handler and a tile handler? would need to not consume the events for that
// and would need to make sure we handle objects before tiles ...
#[allow(clippy::too_many_arguments)]
pub(super) fn item_drag_event_handler(
    mut commands: Commands,
    mut events: EventReader<ItemDragEvent>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut drag_object: Option<ResMut<ItemDragObject>>,
    object_layer_query: Query<TileMapQuery, With<TiledMapObjectLayer>>,
    mut object_query: Query<&mut TileColor, With<Object>>,
    mut drag_tile: Option<ResMut<ItemDragTile>>,
    tilemap_layer_query: Query<TileMapQuery, With<TiledMapTileLayer>>,
    mut tile_query: Query<&mut TileColor, Without<Object>>,
) {
    if events.is_empty() {
        return;
    }

    let (camera, camera_transform) = camera_query.single();

    // TODO: should we just deal with the first (or last?) event?
    // what does it even mean to have more than one of these ...
    for event in events.read() {
        // TODO: pretty sure this is missing some edge cases

        let world_position = get_world_position_from_cursor_position(
            event.cursor_position,
            camera,
            camera_transform,
        );
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
                if let Some(object_entity) = object_tilemap.storage.get(&object_position) {
                    // reset and remove previous tile
                    if let Some(drag_tile) = &drag_tile {
                        let mut color = tile_query.get_mut(drag_tile.0).unwrap();
                        color.0 = Color::default();
                        commands.remove_resource::<ItemDragTile>();
                    }

                    // check previous object
                    if let Some(drag_object) = &mut drag_object {
                        if drag_object.0 != object_entity {
                            let mut color = object_query.get_mut(drag_object.0).unwrap();
                            color.0 = Color::default();

                            let mut color = object_query.get_mut(object_entity).unwrap();
                            color.0 = if event.item_type.can_drop_on_object() {
                                CAN_DROP_COLOR
                            } else {
                                NO_DROP_COLOR
                            };
                            drag_object.0 = object_entity;
                        }
                    } else {
                        let mut color = object_query.get_mut(object_entity).unwrap();
                        color.0 = if event.item_type.can_drop_on_object() {
                            CAN_DROP_COLOR
                        } else {
                            NO_DROP_COLOR
                        };
                        commands.insert_resource(ItemDragObject(object_entity));
                    }
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
                if let Some(tile_entity) = tilemap.storage.get(&tile_position) {
                    // reset and remove previous object
                    if let Some(drag_object) = &drag_object {
                        let mut color = object_query.get_mut(drag_object.0).unwrap();
                        color.0 = Color::default();
                        commands.remove_resource::<ItemDragObject>();
                    }

                    // check previous tile
                    if let Some(drag_tile) = &mut drag_tile {
                        if drag_tile.0 != tile_entity {
                            let mut color = tile_query.get_mut(drag_tile.0).unwrap();
                            color.0 = Color::default();

                            let mut color = tile_query.get_mut(tile_entity).unwrap();
                            color.0 = if event.item_type.can_drop_on_tile() {
                                CAN_DROP_COLOR
                            } else {
                                NO_DROP_COLOR
                            };
                            drag_tile.0 = tile_entity;
                        }
                    } else {
                        let mut color = tile_query.get_mut(tile_entity).unwrap();
                        color.0 = if event.item_type.can_drop_on_tile() {
                            CAN_DROP_COLOR
                        } else {
                            NO_DROP_COLOR
                        };
                        commands.insert_resource(ItemDragTile(tile_entity));
                    }
                    continue;
                }
            }
        }
    }
}

// TODO: we might be able to simplify this by splitting it into
// an object handler and a tile handler? would need to not consume the events for that
// and would need to make sure we handle objects before tiles ...
#[allow(clippy::too_many_arguments)]
pub(super) fn item_drop_event_handler(
    mut commands: Commands,
    mut events: EventReader<ItemDropEvent>,
    mut inventory: ResMut<Inventory>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    drag_object: Option<Res<ItemDragObject>>,
    object_layer_query: Query<TileMapQuery, With<TiledMapObjectLayer>>,
    mut object_query: Query<&mut TileColor, With<Object>>,
    drag_tile: Option<Res<ItemDragTile>>,
    tilemap_layer_query: Query<TileMapQuery, With<TiledMapTileLayer>>,
    mut tile_query: Query<&mut TileColor, Without<Object>>,
    mut inventory_updated_events: EventWriter<InventoryUpdatedEvent>,
) {
    if events.is_empty() {
        return;
    }

    let (camera, camera_transform) = camera_query.single();

    // TODO: should we just deal with the first (or last?) event?
    // what does it even mean to have more than one of these ...
    for event in events.read() {
        // TODO: pretty sure this is missing some edge cases

        let world_position = get_world_position_from_cursor_position(
            event.cursor_position,
            camera,
            camera_transform,
        );
        if let Some(world_position) = world_position {
            // first check for objects
            if let Some(drag_object) = &drag_object {
                let mut color = object_query.get_mut(drag_object.0).unwrap();
                color.0 = Color::default();
                commands.remove_resource::<ItemDragObject>();

                let object_tilemap = object_layer_query.single();
                let object_position = get_tile_position(
                    world_position,
                    object_tilemap.size,
                    object_tilemap.grid_size,
                    object_tilemap.r#type,
                    object_tilemap.transform,
                )
                .unwrap();

                let _object_entity = object_tilemap.storage.get(&object_position).unwrap();
                if event.item_type.can_drop_on_object() {
                    event
                        .item_type
                        .drop_object(&mut inventory.0, &mut inventory_updated_events);
                }

                continue;
            }

            // then check for tiles
            if let Some(drag_tile) = &drag_tile {
                let mut color = tile_query.get_mut(drag_tile.0).unwrap();
                color.0 = Color::default();
                commands.remove_resource::<ItemDragTile>();

                let tilemap = tilemap_layer_query.single();
                let tile_position = get_tile_position(
                    world_position,
                    tilemap.size,
                    tilemap.grid_size,
                    tilemap.r#type,
                    tilemap.transform,
                )
                .unwrap();

                let _tile_entity = tilemap.storage.get(&tile_position).unwrap();
                if event.item_type.can_drop_on_tile() {
                    event
                        .item_type
                        .drop_tile(&mut inventory.0, &mut inventory_updated_events);
                } else {
                    let tween = bevy_tweening::Tween::new(
                        bevy_tweening::EaseFunction::QuadraticOut,
                        std::time::Duration::from_secs(1),
                        bevy_tweening::lens::UiPositionLens {
                            start: UiRect {
                                left: event.drag_image_position.0,
                                top: event.drag_image_position.1,
                                right: Val::Auto,
                                bottom: Val::Auto,
                            },
                            // TODO: this should be the drag start
                            end: UiRect {
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                right: Val::Auto,
                                bottom: Val::Auto,
                            },
                        },
                    );
                    // TODO: hmmmmmm
                    //.with_completed_system(hide_inventory_drag_icon);

                    commands
                        .entity(event.drag_image_id)
                        .insert(bevy_tweening::Animator::new(tween));
                }

                continue;
            }
        }
    }
}
