use bevy::{ecs::query::QueryData, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::data::objects::ObjectData;
use crate::plugins::game::objects::Object;
use crate::plugins::tiled::{TiledMapItemClickEvent, TiledMapObjectClickEvent};

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

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct TileMapQueryMut {
    pub size: &'static TilemapSize,
    pub grid_size: &'static TilemapGridSize,
    pub r#type: &'static TilemapType,
    pub storage: &'static mut TileStorage,
    pub transform: &'static Transform,
}

#[inline]
pub fn get_tile_position(
    world_position: Vec2,
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    map_transform: &Transform,
) -> Option<TilePos> {
    let map_position = {
        let world_position = Vec4::from((world_position, 0.0, 1.0));
        let map_position = map_transform.compute_matrix().inverse() * world_position;
        map_position.xy()
    };

    TilePos::from_world_pos(&map_position, map_size, grid_size, map_type)
}

pub fn spawn_tile(
    parent: &mut ChildBuilder,
    storage: &mut TileStorage,
    tilemap_id: Entity,
    position: TilePos,
    texture_index: u32,
    visible: bool,
) -> Entity {
    let tile_entity = parent
        .spawn((
            TileBundle {
                position,
                tilemap_id: TilemapId(tilemap_id),
                texture_index: TileTextureIndex(texture_index),
                visible: TileVisible(visible),
                ..Default::default()
            },
            Name::new(format!("Tile ({},{})", position.x, position.y)),
        ))
        .id();

    storage.set(&position, tile_entity);

    tile_entity
}

pub fn despawn_tile(
    commands: &mut Commands,
    storage: &mut TileStorage,
    tile_id: Entity,
    position: TilePos,
) {
    commands.entity(tile_id).despawn_recursive();
    storage.remove(&position);
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_object(
    parent: &mut ChildBuilder,
    storage: &mut TileStorage,
    tilemap_id: Entity,
    position: TilePos,
    texture_index: u32,
    visible: bool,
    data: ObjectData,
) -> Entity {
    let tile_entity = parent
        .spawn((
            TileBundle {
                position,
                tilemap_id: TilemapId(tilemap_id),
                texture_index: TileTextureIndex(texture_index),
                visible: TileVisible(visible),
                ..Default::default()
            },
            Name::new(format!("Object ({},{})", position.x, position.y)),
            Object(data),
            PickableBundle::default(),
            On::<Pointer<Click>>::run(
                |event: Listener<Pointer<Click>>,
                 mut click_events: EventWriter<TiledMapObjectClickEvent>| {
                    click_events.send(TiledMapObjectClickEvent {
                        listener: event.listener(),
                        target: event.target,
                        button: event.button,
                    });
                },
            ),
        ))
        .id();

    storage.set(&position, tile_entity);

    tile_entity
}

pub fn despawn_object(
    commands: &mut Commands,
    storage: &mut TileStorage,
    object_id: Entity,
    position: TilePos,
) {
    commands.entity(object_id).despawn_recursive();
    storage.remove(&position);
}

#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
pub fn spawn_item<C>(
    parent: &mut ChildBuilder,
    storage: &mut TileStorage,
    tilemap_id: Entity,
    position: TilePos,
    texture_index: u32,
    visible: bool,
    tag: C,
) -> Entity
where
    C: Component,
{
    let tile_entity = parent
        .spawn((
            TileBundle {
                position,
                tilemap_id: TilemapId(tilemap_id),
                texture_index: TileTextureIndex(texture_index),
                visible: TileVisible(visible),
                ..Default::default()
            },
            Name::new(format!("Item ({},{})", position.x, position.y)),
            tag,
            PickableBundle::default(),
            On::<Pointer<Click>>::run(
                |event: Listener<Pointer<Click>>,
                 mut click_events: EventWriter<TiledMapItemClickEvent>| {
                    click_events.send(TiledMapItemClickEvent {
                        listener: event.listener(),
                        target: event.target,
                        button: event.button,
                    });
                },
            ),
        ))
        .id();

    storage.set(&position, tile_entity);

    tile_entity
}

#[allow(dead_code)]
pub fn despawn_item(
    commands: &mut Commands,
    storage: &mut TileStorage,
    item_id: Entity,
    position: TilePos,
) {
    commands.entity(item_id).despawn_recursive();
    storage.remove(&position);
}
