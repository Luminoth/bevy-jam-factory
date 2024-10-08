use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::backend::prelude::*;

use crate::get_world_position_from_cursor_position;
use crate::plugins::{game::camera::MainCamera, tiled::TiledMapObjectLayer};
use crate::tilemap::{get_tile_position, TileMapQuery};

#[derive(Debug, Default)]
pub struct TiledPickingBackend;

impl Plugin for TiledPickingBackend {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, object_picking.in_set(PickSet::Backend));
    }
}

fn object_picking(
    pointers: Query<(&PointerId, &PointerLocation)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(Entity, &Camera, &GlobalTransform), With<MainCamera>>,
    object_layer_query: Query<TileMapQuery, With<TiledMapObjectLayer>>,
    mut output: EventWriter<PointerHits>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let Ok((camera_entity, camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let Ok(object_tilemap) = object_layer_query.get_single() else {
        return;
    };

    for (pointer_id, pointer_location) in
        pointers.iter().filter_map(|(pointer, pointer_location)| {
            Some(*pointer).zip(pointer_location.location().cloned())
        })
    {
        let mut pointer_pos = pointer_location.position;
        if let Some(viewport) = camera.logical_viewport_rect() {
            pointer_pos -= viewport.min;
        }

        if let Some(world_position) = get_world_position_from_cursor_position(
            window.cursor_position(),
            camera,
            camera_transform,
        ) {
            if let Some(object_position) = get_tile_position(
                world_position,
                object_tilemap.size,
                object_tilemap.grid_size,
                object_tilemap.r#type,
                object_tilemap.transform,
            ) {
                if let Some(tile_entity) = object_tilemap.storage.get(&object_position) {
                    // TODO: don't pick objects that aren't visible
                    // (have to query TileVisible to check this)

                    output.send(PointerHits::new(
                        pointer_id,
                        vec![(
                            tile_entity,
                            HitData::new(
                                camera_entity,
                                0.0,
                                Some(world_position.extend(0.0)),
                                None,
                            ),
                        )],
                        camera.order as f32,
                    ));
                }
            }
        }
    }
}
