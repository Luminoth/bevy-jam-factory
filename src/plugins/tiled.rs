use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::assets::tiled::*;
use crate::data::objects::ObjectData;
use crate::plugins::game::OnInGame;
use crate::tilemap::{spawn_object, spawn_tile};

/// Maps layer index to layer id
///
/// Tiles are created as children
/// of their respective layer entities
#[derive(Debug, Default, Component)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}

// TODO: this shouldn't need to be a bundle,
// we only load a single map at a time
// so the storage and render settings
// could be stored in a resource ?
#[derive(Debug, Default, Bundle)]
pub struct TiledMapBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    // TODO: this just straight up doesn't work if this handle isn't in here
    pub tiled_map: Handle<TiledMap>,
    pub storage: TiledLayersStorage,
    pub render_settings: TilemapRenderSettings,
}

/// Tile layer tag
#[derive(Debug, Component)]
pub struct TiledMapTileLayer;

/// Object layer tag
#[derive(Debug, Component)]
pub struct TiledMapObjectLayer;

/// Item layer tag
#[derive(Debug, Component)]
pub struct TiledMapItemLayer;

/// Emitted when an Object is clicked
#[derive(Debug, Event)]
pub struct TiledMapObjectClickEvent {
    pub listener: Entity,
    pub target: Entity,
    pub button: PointerButton,
}

/// Emitted when an Item is clicked
#[allow(dead_code)]
#[derive(Debug, Event)]
pub struct TiledMapItemClickEvent {
    pub listener: Entity,
    pub target: Entity,
    pub button: PointerButton,
}

const MIN_TILEMAP_WIDTH: u32 = 25;
const MIN_TILEMAP_HEIGHT: u32 = 25;
const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;

#[derive(Debug, Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TiledMap>()
            .register_asset_loader(TiledLoader)
            .add_event::<TiledMapObjectClickEvent>()
            .add_event::<TiledMapItemClickEvent>()
            .add_systems(Update, process_loaded_maps);
    }
}

// TODO: how does this even work ... the asset is loaded
// before the bundle is available, right?
// so it's just getting lucky on the timing?
fn process_loaded_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<TiledMap>>,
    tiled_maps: Res<Assets<TiledMap>>,
    tile_storage_query: Query<(Entity, &TileStorage)>,
    mut tiled_map_query: Query<(
        &Handle<TiledMap>,
        &mut TiledLayersStorage,
        &TilemapRenderSettings,
    )>,
    //new_maps: Query<&Handle<TiledMap>, Added<Handle<TiledMap>>>,
) {
    let mut changed_maps = Vec::<AssetId<TiledMap>>::default();
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                debug!("Map {} added", id);
                changed_maps.push(*id);
            }
            AssetEvent::Modified { id } => {
                debug!("Map {} changed", id);
                changed_maps.push(*id);
            }
            AssetEvent::Removed { id } => {
                debug!("Map {} removed", id);
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.retain(|changed_handle| changed_handle == id);
            }
            _ => continue,
        }
    }

    // If we have new map entities add them to the changed_maps list.
    // NOTE: not sure why this is done, the Added event should be handling this?
    // and doing this causes us to process new maps twice
    /*for new_map_handle in new_maps.iter() {
        debug!("New map {} added", new_map_handle.id());
        changed_maps.push(new_map_handle.id());
    }*/

    for changed_map in changed_maps.iter() {
        for (map_handle, mut layer_storage, render_settings) in tiled_map_query.iter_mut() {
            // only deal with currently changed map
            if map_handle.id() != *changed_map {
                continue;
            }

            debug!("Processing map {}", map_handle.id());

            if let Some(tiled_map) = tiled_maps.get(map_handle) {
                // TODO: Create a RemoveMap component..
                for layer_entity in layer_storage.storage.values() {
                    if let Ok((_, layer_tile_storage)) = tile_storage_query.get(*layer_entity) {
                        for tile in layer_tile_storage.iter().flatten() {
                            commands.entity(*tile).despawn_recursive()
                        }
                    }
                    // commands.entity(*layer_entity).despawn_recursive();
                }

                process_loaded_map(
                    &mut commands,
                    tiled_map,
                    &mut layer_storage,
                    *render_settings,
                );
            }
        }
    }
}

fn process_loaded_map(
    commands: &mut Commands,
    tiled_map: &TiledMap,
    layer_storage: &mut TiledLayersStorage,
    render_settings: TilemapRenderSettings,
) {
    debug!("Processing loaded map {}", tiled_map.name);

    if tiled_map.map.width < MIN_TILEMAP_WIDTH || tiled_map.map.height < MIN_TILEMAP_HEIGHT {
        panic!("Map {} is below the minimum size", tiled_map.name);
    }

    if tiled_map.map.tile_width != TILE_WIDTH || tiled_map.map.tile_height != TILE_HEIGHT {
        panic!("Map {} tiles have invalid tile size", tiled_map.name);
    }

    commands
        .spawn((
            SpatialBundle::default(),
            Name::new(tiled_map.name.clone()),
            OnInGame,
        ))
        .with_children(|parent| {
            // TODO: better explain the way this is restricted and what we're doing about it
            //
            // The TilemapBundle requires that all tile images come exclusively from a single
            // tiled texture or from a Vec of independent per-tile images. Furthermore, all of
            // the per-tile images must be the same size. Since Tiled allows tiles of mixed
            // tilesets on each layer and allows differently-sized tile images in each tileset,
            // this means we need to load each combination of tileset and layer separately.

            // Once materials have been created/added we need to then create the layers.
            for (layer_index, layer) in tiled_map.map.layers().enumerate() {
                debug!("Processing layer {} ({}) ", layer_index, layer.id());

                if layer.offset_x != 0.0 || layer.offset_y != 0.0 {
                    panic!("Layer {} has invalid offset", layer.id());
                }

                match layer.layer_type() {
                    tiled::LayerType::Tiles(tile_layer) => {
                        process_tile_layer(
                            parent,
                            layer_storage,
                            tiled_map,
                            layer_index,
                            layer.id(),
                            &tile_layer,
                            render_settings,
                        );
                    }
                    tiled::LayerType::Objects(object_layer) => {
                        process_object_layer(
                            parent,
                            layer_storage,
                            tiled_map,
                            layer_index,
                            layer.id(),
                            &object_layer,
                            render_settings,
                        );
                    }
                    _ => {
                        panic!(
                            "Invalid layer {} - only Tile and Object layers are supported.",
                            layer.id()
                        );
                    }
                }
            }

            create_item_layer(parent, layer_storage, tiled_map, render_settings);
        });
}

#[allow(clippy::too_many_arguments)]
fn process_tile_layer(
    parent: &mut ChildBuilder,
    layer_storage: &mut TiledLayersStorage,
    tiled_map: &TiledMap,
    layer_index: usize,
    layer_id: u32,
    layer: &tiled::TileLayer,
    render_settings: TilemapRenderSettings,
) {
    debug!("Processing tile layer {} ({})", layer_index, layer_id);

    let tiled::TileLayer::Finite(layer) = layer else {
        panic!("Tile layer {} may not be infinite", layer_id);
    };

    let map_size = TilemapSize {
        x: tiled_map.map.width,
        y: tiled_map.map.height,
    };

    let grid_size = TilemapGridSize {
        x: tiled_map.map.tile_width as f32,
        y: tiled_map.map.tile_height as f32,
    };

    let map_type = match tiled_map.map.orientation {
        tiled::Orientation::Orthogonal => TilemapType::Square,
        _ => panic!("Tile layer {} must be Orthogonal", layer_id),
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let mut layer_entity = parent.spawn((
        SpatialBundle::default(),
        Name::new(format!("Tile Layer {}", layer_id)),
    ));
    let layer_entity_id = layer_entity.id();

    let mut shared_tilemap_texture = None;

    layer_entity.with_children(|parent| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                // Transform TMX coords into bevy coords.
                let mapped_y = tiled_map.map.height - 1 - y;

                let mapped_x = x as i32;
                let mapped_y = mapped_y as i32;

                let layer_tile = layer.get_tile(mapped_x, mapped_y).unwrap_or_else(|| {
                    panic!("Tile layer {} missing tile at ({}, {})", layer_id, x, y)
                });

                let tileset = layer_tile.get_tileset();
                if tileset.tile_width != TILE_WIDTH || tileset.tile_height != TILE_HEIGHT {
                    panic!("Tileset {} tiles have invalid tile size", tileset.name);
                }

                if tileset.spacing != 0 {
                    panic!("Tileset {} tiles have invalid tile spacing", tileset.name);
                }

                let tilemap_texture = tiled_map
                    .tilemap_textures
                    .get(&tileset.name)
                    .unwrap_or_else(|| panic!("Tile layer {} missing tilemap texture", layer_id));

                if shared_tilemap_texture.is_none() {
                    shared_tilemap_texture = Some(tilemap_texture.clone());
                }
                // TODO: ensure the textures are the same texture

                let layer_tile_data =
                    layer.get_tile_data(mapped_x, mapped_y).unwrap_or_else(|| {
                        panic!(
                            "Tile layer {} missing tile data at ({}, {})",
                            layer_id, x, y
                        )
                    });

                if layer_tile_data.flip_h || layer_tile_data.flip_v || layer_tile_data.flip_d {
                    panic!(
                        "Tile layer {} has unsupported tile flip at ({}, {})",
                        layer_id, x, y
                    )
                }

                let texture_index = match tilemap_texture {
                    TilemapTexture::Single(_) => layer_tile.id(),
                };

                let tile_pos = TilePos { x, y };
                spawn_tile(
                    parent,
                    &mut tile_storage,
                    layer_entity_id,
                    tile_pos,
                    texture_index,
                    true,
                );
            }
        }
    });

    layer_entity.insert((
        TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            texture: shared_tilemap_texture.unwrap(),
            tile_size: TilemapTileSize {
                x: TILE_WIDTH as f32,
                y: TILE_HEIGHT as f32,
            },
            transform: get_tilemap_center_transform(
                &map_size,
                &grid_size,
                &map_type,
                layer_index as f32,
            ),
            map_type,
            render_settings,
            ..Default::default()
        },
        TiledMapTileLayer,
    ));

    layer_storage
        .storage
        .insert(layer_index as u32, layer_entity_id);
}

// TODO: Objects should be allowed in multiples of the tile size
// (so that we can have small and large objects as needed)
#[allow(clippy::too_many_arguments)]
fn process_object_layer(
    parent: &mut ChildBuilder,
    layer_storage: &mut TiledLayersStorage,
    tiled_map: &TiledMap,
    layer_index: usize,
    layer_id: u32,
    layer: &tiled::ObjectLayer,
    render_settings: TilemapRenderSettings,
) {
    debug!("Processing object layer {} ({})", layer_index, layer_id);

    let map_size = TilemapSize {
        x: tiled_map.map.width,
        y: tiled_map.map.height,
    };

    let grid_size = TilemapGridSize {
        x: tiled_map.map.tile_width as f32,
        y: tiled_map.map.tile_height as f32,
    };

    let map_type = match tiled_map.map.orientation {
        tiled::Orientation::Orthogonal => TilemapType::Square,
        _ => panic!("Object layer {} must be Orthogonal", layer_id),
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let mut layer_entity = parent.spawn((
        SpatialBundle::default(),
        Name::new(format!("Object layer {}", layer_id)),
    ));
    let layer_entity_id = layer_entity.id();

    let mut shared_tilemap_texture = None;

    layer_entity.with_children(|parent| {
        for object in layer.objects() {
            if object.rotation != 0.0 {
                panic!(
                    "Object layer {} may not have rotated object {}",
                    layer_id,
                    object.id()
                );
            }

            let object_tile = object.get_tile().unwrap_or_else(|| {
                panic!(
                    "Object layer {} missing tile for object {}",
                    layer_id,
                    object.id()
                )
            });

            let tileset = object_tile.get_tileset();
            if tileset.tile_width != TILE_WIDTH || tileset.tile_height != TILE_HEIGHT {
                panic!(
                    "Object tileset {} tiles have invalid tile size",
                    tileset.name
                );
            }

            if tileset.spacing != 0 {
                panic!(
                    "Object tileset {} tiles have invalid tile spacing",
                    tileset.name
                );
            }

            let tilemap_texture = tiled_map
                .tilemap_textures
                .get(&tileset.name)
                .unwrap_or_else(|| {
                    panic!(
                        "Object layer {} missing tilemap texture for object {}",
                        layer_id,
                        object.id()
                    )
                });

            if shared_tilemap_texture.is_none() {
                shared_tilemap_texture = Some(tilemap_texture.clone());
            }
            // TODO: ensure the textures are the same texture

            let object_tile_data = object.tile_data().unwrap_or_else(|| {
                panic!(
                    "Object layer {} missing tile data for object {}",
                    layer_id,
                    object.id()
                )
            });

            if object_tile_data.flip_h || object_tile_data.flip_v || object_tile_data.flip_d {
                panic!(
                    "Object layer {} has unsupported tile flip for object {}",
                    layer_id,
                    object.id()
                )
            }

            let texture_index = match tilemap_texture {
                TilemapTexture::Single(_) => object_tile.id(),
            };

            let (x, y) = match object.shape {
                tiled::ObjectShape::Rect { width, height } => {
                    if width != TILE_WIDTH as f32 || height != TILE_HEIGHT as f32 {
                        panic!("Object {} shape has invalid size", object.id());
                    }
                    (object.x / width, object.y / height)
                }
                _ => {
                    panic!(
                        "Object layer {} has unsupported shape {:?} for object {}",
                        layer_id,
                        object.shape,
                        object.id(),
                    );
                }
            };

            let object_data = ObjectData::new(layer_id, &object)
                .unwrap_or_else(|err| panic!("Object {} failed to load: {}", object.id(), err,));

            let tile_pos = TilePos {
                x: x as u32,

                // Transform TMX coords into bevy coords.
                y: tiled_map.map.height - 1 - y as u32,
            };

            spawn_object(
                parent,
                &mut tile_storage,
                layer_entity_id,
                tile_pos,
                texture_index,
                object.visible,
                object_data,
            );
        }
    });

    layer_entity.insert((
        TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            texture: shared_tilemap_texture.unwrap(),
            tile_size: TilemapTileSize {
                x: TILE_WIDTH as f32,
                y: TILE_HEIGHT as f32,
            },
            transform: get_tilemap_center_transform(
                &map_size,
                &grid_size,
                &map_type,
                layer_index as f32,
            ),
            map_type,
            render_settings,
            ..Default::default()
        },
        TiledMapObjectLayer,
    ));

    layer_storage
        .storage
        .insert(layer_index as u32, layer_entity_id);
}

// TODO: Items should be allowed in multiples of the tile size
// (so that we can have small and large objects as needed)
#[allow(clippy::too_many_arguments)]
fn create_item_layer(
    parent: &mut ChildBuilder,
    _layer_storage: &mut TiledLayersStorage,
    tiled_map: &TiledMap,
    _render_settings: TilemapRenderSettings,
) {
    let layer_index = tiled_map.map.layers().len() + 1;
    let layer_id = 0; // TODO: use an actual unique id
    debug!("Creating item layer {} ({})", layer_index, layer_id);

    let map_size = TilemapSize {
        x: tiled_map.map.width,
        y: tiled_map.map.height,
    };

    let mut _tile_storage = TileStorage::empty(map_size);
    let mut layer_entity = parent.spawn((
        SpatialBundle::default(),
        Name::new(format!("Item layer {}", layer_id)),
    ));

    // TODO: fix this
    layer_entity.insert((
        /*TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            texture: shared_tilemap_texture.unwrap(),
            tile_size,
            spacing: tile_spacing,
            transform: get_tilemap_center_transform(
                &map_size,
                &grid_size,
                &map_type,
                layer_index as f32,
            ) * Transform::from_xyz(offset.0, -offset.1, 0.0),
            map_type,
            render_settings,
            ..Default::default()
        },*/
        TiledMapItemLayer,
    ));
}
