use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::assets::tiled::*;
use crate::components::tiled::*;

pub fn process_loaded_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<TiledMap>>,
    maps: Res<Assets<TiledMap>>,
    tile_storage_query: Query<(Entity, &TileStorage)>,
    mut map_query: Query<(
        &Handle<TiledMap>,
        &mut TiledLayersStorage,
        &TilemapRenderSettings,
    )>,
    new_maps: Query<&Handle<TiledMap>, Added<Handle<TiledMap>>>,
) {
    let mut changed_maps = Vec::<AssetId<TiledMap>>::default();
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                info!("Map added!");
                changed_maps.push(*id);
            }
            AssetEvent::Modified { id } => {
                info!("Map changed!");
                changed_maps.push(*id);
            }
            AssetEvent::Removed { id } => {
                info!("Map removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.retain(|changed_handle| changed_handle == id);
            }
            _ => continue,
        }
    }

    // If we have new map entities add them to the changed_maps list.
    for new_map_handle in new_maps.iter() {
        changed_maps.push(new_map_handle.id());
    }

    for changed_map in changed_maps.iter() {
        for (map_handle, mut layer_storage, render_settings) in map_query.iter_mut() {
            // only deal with currently changed map
            if map_handle.id() != *changed_map {
                continue;
            }

            if let Some(tiled_map) = maps.get(map_handle) {
                // TODO: Create a RemoveMap component..
                for layer_entity in layer_storage.storage.values() {
                    if let Ok((_, layer_tile_storage)) = tile_storage_query.get(*layer_entity) {
                        for tile in layer_tile_storage.iter().flatten() {
                            commands.entity(*tile).despawn_recursive()
                        }
                    }
                    // commands.entity(*layer_entity).despawn_recursive();
                }

                // The TilemapBundle requires that all tile images come exclusively from a single
                // tiled texture or from a Vec of independent per-tile images. Furthermore, all of
                // the per-tile images must be the same size. Since Tiled allows tiles of mixed
                // tilesets on each layer and allows differently-sized tile images in each tileset,
                // this means we need to load each combination of tileset and layer separately.
                for (tileset_index, tileset) in tiled_map.map.tilesets().iter().enumerate() {
                    // Once materials have been created/added we need to then create the layers.
                    for (layer_index, layer) in tiled_map.map.layers().enumerate() {
                        let offset_x = layer.offset_x;
                        let offset_y = layer.offset_y;

                        match layer.layer_type() {
                            tiled::LayerType::Tiles(tile_layer) => {
                                process_tile_layer(
                                    &mut commands,
                                    &mut layer_storage,
                                    tiled_map,
                                    tileset_index,
                                    tileset,
                                    layer_index,
                                    layer.id(),
                                    &tile_layer,
                                    (offset_x, offset_y),
                                    *render_settings,
                                );
                            }
                            tiled::LayerType::Objects(object_layer) => {
                                process_object_layer(layer.id(), &object_layer);
                            }
                            _ => {
                                info!(
                                    "Skipping layer {} because only tile layers are supported.",
                                    layer.id()
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn process_tile_layer(
    commands: &mut Commands,
    layer_storage: &mut TiledLayersStorage,
    tiled_map: &TiledMap,
    tileset_index: usize,
    tileset: &tiled::Tileset,
    layer_index: usize,
    layer_id: u32,
    layer: &tiled::TileLayer,
    offset: (f32, f32),
    render_settings: TilemapRenderSettings,
) {
    let tiled::TileLayer::Finite(layer_data) = layer else {
        info!(
            "Skipping layer {} because only finite layers are supported.",
            layer_id,
        );
        return;
    };

    let Some(tilemap_texture) = tiled_map.tilemap_textures.get(&tileset_index) else {
        warn!("Skipped creating layer with missing tilemap textures.");
        return;
    };

    let map_size = TilemapSize {
        x: tiled_map.map.width,
        y: tiled_map.map.height,
    };

    let grid_size = TilemapGridSize {
        x: tiled_map.map.tile_width as f32,
        y: tiled_map.map.tile_height as f32,
    };

    let tile_size = TilemapTileSize {
        x: tileset.tile_width as f32,
        y: tileset.tile_height as f32,
    };

    let tile_spacing = TilemapSpacing {
        x: tileset.spacing as f32,
        y: tileset.spacing as f32,
    };

    let map_type = match tiled_map.map.orientation {
        tiled::Orientation::Hexagonal => TilemapType::Hexagon(HexCoordSystem::Row),
        tiled::Orientation::Isometric => TilemapType::Isometric(IsoCoordSystem::Diamond),
        tiled::Orientation::Staggered => TilemapType::Isometric(IsoCoordSystem::Staggered),
        tiled::Orientation::Orthogonal => TilemapType::Square,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let layer_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            // Transform TMX coords into bevy coords.
            let mapped_y = tiled_map.map.height - 1 - y;

            let mapped_x = x as i32;
            let mapped_y = mapped_y as i32;

            let layer_tile = match layer_data.get_tile(mapped_x, mapped_y) {
                Some(t) => t,
                None => {
                    continue;
                }
            };

            if tileset_index != layer_tile.tileset_index() {
                continue;
            }

            let layer_tile_data = match layer_data.get_tile_data(mapped_x, mapped_y) {
                Some(d) => d,
                None => continue,
            };

            let texture_index = match tilemap_texture {
                TilemapTexture::Single(_) => layer_tile.id(),
                #[cfg(not(feature = "atlas"))]
                TilemapTexture::Vector(_) =>
                    *tiled_map.tile_image_offsets.get(&(tileset_index, layer_tile.id()))
                    .expect("The offset into to image vector should have been saved during the initial load."),
                #[cfg(not(feature = "atlas"))]
                _ => unreachable!()
            };

            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(layer_entity),
                    texture_index: TileTextureIndex(texture_index),
                    flip: TileFlip {
                        x: layer_tile_data.flip_h,
                        y: layer_tile_data.flip_v,
                        d: layer_tile_data.flip_d,
                    },
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(layer_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: tilemap_texture.clone(),
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
    });

    layer_storage
        .storage
        .insert(layer_index as u32, layer_entity);
}

fn process_object_layer(layer_id: u32, _layer: &tiled::ObjectLayer) {
    info!(
        "Skipping layer {} because object layers are not supported.",
        layer_id,
    );
}
