use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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
