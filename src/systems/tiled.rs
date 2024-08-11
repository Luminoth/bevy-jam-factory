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
        for (map_handle, mut layer_storage, render_settings) in map_query.iter_mut() {
            // only deal with currently changed map
            if map_handle.id() != *changed_map {
                continue;
            }

            debug!("Processing map {}", map_handle.id());

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

    commands
        .spawn(Name::new(tiled_map.name.clone()))
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
                debug!("Processing layer {} ({}) ", layer_index, layer.id(),);

                match layer.layer_type() {
                    tiled::LayerType::Tiles(tile_layer) => {
                        process_tile_layer(
                            parent,
                            layer_storage,
                            tiled_map,
                            layer_index,
                            layer.id(),
                            &tile_layer,
                            (layer.offset_x, layer.offset_y),
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
                            (layer.offset_x, layer.offset_y),
                            render_settings,
                        );
                    }
                    _ => {
                        warn!(
                            "Skipping layer {} because only tile layers are supported.",
                            layer.id()
                        );
                        continue;
                    }
                }
            }
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
    offset: (f32, f32),
    render_settings: TilemapRenderSettings,
) {
    debug!("Processing tile layer {} ({})", layer_index, layer_id);

    let tiled::TileLayer::Finite(layer) = layer else {
        warn!(
            "Skipping layer {} because only finite layers are supported.",
            layer_id,
        );
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

    let map_type = match tiled_map.map.orientation {
        tiled::Orientation::Hexagonal => TilemapType::Hexagon(HexCoordSystem::Row),
        tiled::Orientation::Isometric => TilemapType::Isometric(IsoCoordSystem::Diamond),
        tiled::Orientation::Staggered => TilemapType::Isometric(IsoCoordSystem::Staggered),
        tiled::Orientation::Orthogonal => TilemapType::Square,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let mut layer_entity = parent.spawn(Name::new(format!("Tile Layer {}", layer_id)));
    let layer_entity_id = layer_entity.id();

    let mut shared_tilemap_texture = None;
    let mut tile_size = TilemapTileSize::default();
    let mut tile_spacing = TilemapSpacing::default();

    layer_entity.with_children(|parent| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                // Transform TMX coords into bevy coords.
                let mapped_y = tiled_map.map.height - 1 - y;

                let mapped_x = x as i32;
                let mapped_y = mapped_y as i32;

                let layer_tile = match layer.get_tile(mapped_x, mapped_y) {
                    Some(t) => t,
                    None => {
                        continue;
                    }
                };

                let tileset = layer_tile.get_tileset();

                let Some(tilemap_texture) = tiled_map.tilemap_textures.get(&tileset.name) else {
                    warn!("Skipped creating layer with missing tilemap textures.");
                    return;
                };

                if shared_tilemap_texture.is_none() {
                    shared_tilemap_texture = Some(tilemap_texture.clone());
                }

                tile_size = TilemapTileSize {
                    x: tileset.tile_width as f32,
                    y: tileset.tile_height as f32,
                };

                tile_spacing = TilemapSpacing {
                    x: tileset.spacing as f32,
                    y: tileset.spacing as f32,
                };

                let layer_tile_data = match layer.get_tile_data(mapped_x, mapped_y) {
                    Some(d) => d,
                    None => continue,
                };

                let texture_index = match tilemap_texture {
                    TilemapTexture::Single(_) => layer_tile.id(),
                    #[cfg(not(feature = "atlas"))]
                    TilemapTexture::Vector(_) =>
                        // TODO: this string clone is so bad :(
                        *tiled_map.tile_image_offsets.get(&(tileset.name.clone(), layer_tile.id()))
                        .expect("The offset into to image vector should have been saved during the initial load."),
                    #[cfg(not(feature = "atlas"))]
                    _ => unreachable!()
                };

                let tile_pos = TilePos { x, y };
                let tile_entity = parent
                    .spawn((TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(layer_entity_id),
                            texture_index: TileTextureIndex(texture_index),
                            flip: TileFlip {
                                x: layer_tile_data.flip_h,
                                y: layer_tile_data.flip_v,
                                d: layer_tile_data.flip_d,
                            },
                            ..Default::default()
                        },
                        Name::new(format!("Tile ({},{})", x, y))),
                    )
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

    layer_entity.insert(TilemapBundle {
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
    });

    layer_storage
        .storage
        .insert(layer_index as u32, layer_entity_id);
}

fn process_object_layer(
    parent: &mut ChildBuilder,
    layer_storage: &mut TiledLayersStorage,
    tiled_map: &TiledMap,
    layer_index: usize,
    layer_id: u32,
    layer: &tiled::ObjectLayer,
    offset: (f32, f32),
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
        tiled::Orientation::Hexagonal => TilemapType::Hexagon(HexCoordSystem::Row),
        tiled::Orientation::Isometric => TilemapType::Isometric(IsoCoordSystem::Diamond),
        tiled::Orientation::Staggered => TilemapType::Isometric(IsoCoordSystem::Staggered),
        tiled::Orientation::Orthogonal => TilemapType::Square,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let mut layer_entity = parent.spawn(Name::new(format!("Object layer {}", layer_id)));
    let layer_entity_id = layer_entity.id();

    let mut shared_tilemap_texture = None;
    let mut tile_size = TilemapTileSize::default();
    let mut tile_spacing = TilemapSpacing::default();

    layer_entity.with_children(|parent|  {
        for object in layer.objects() {
            let object_tile = match object.get_tile() {
                Some(t) => t,
                None => {
                    continue;
                }
            };

            let tileset = object_tile.get_tileset();

            let Some(tilemap_texture) = tiled_map.tilemap_textures.get(&tileset.name) else {
                warn!("Skipped creating layer with missing tilemap textures.");
                return;
            };

            if shared_tilemap_texture.is_none() {
                shared_tilemap_texture = Some(tilemap_texture.clone());
            }

            tile_size = TilemapTileSize {
                x: tileset.tile_width as f32,
                y: tileset.tile_height as f32,
            };

            tile_spacing = TilemapSpacing {
                x: tileset.spacing as f32,
                y: tileset.spacing as f32,
            };

            let object_tile_data = match object.tile_data() {
                Some(d) => d,
                None => continue,
            };

            let texture_index = match tilemap_texture {
                    TilemapTexture::Single(_) => object_tile.id(),
                    #[cfg(not(feature = "atlas"))]
                    TilemapTexture::Vector(_) =>
                        // TODO: this string clone is so bad :(
                        *tiled_map.tile_image_offsets.get(&(tileset.name.clone(), object_tile.id()))
                        .expect("The offset into to image vector should have been saved during the initial load."),
                    #[cfg(not(feature = "atlas"))]
                    _ => unreachable!()
                };

            let (x, y) = match object.shape {
                tiled::ObjectShape::Rect { width, height } => (object.x / width, object.y / height),
                _ => {
                    warn!(
                        "Skipped object {} in layer {} for unsupported shape {:?}",
                        object.id(),
                        layer_id,
                        object.shape
                    );
                    continue;
                }
            };

            let tile_pos = TilePos {
                x: x as u32,
                y: tiled_map.map.height - 1 - y as u32,
            };
            let tile_entity = parent
                .spawn((TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(layer_entity_id),
                        texture_index: TileTextureIndex(texture_index),
                        flip: TileFlip {
                            x: object_tile_data.flip_h,
                            y: object_tile_data.flip_v,
                            d: object_tile_data.flip_d,
                        },
                        ..Default::default()
                    },
                    Name::new(format!("Object ({},{})", x, y))),
                )
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    });

    layer_entity.insert(TilemapBundle {
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
    });

    layer_storage
        .storage
        .insert(layer_index as u32, layer_entity_id);
}
