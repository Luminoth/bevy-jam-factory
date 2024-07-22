use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), Name::new("Main Camera")));

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 128, y: 128 };
    let map_type = TilemapType::Square;

    let mut tilemap_entity = commands.spawn(Name::new("Tilemap"));
    let tilemap_id = TilemapId(tilemap_entity.id());

    let mut tile_storage = TileStorage::empty(map_size);
    tilemap_entity.with_children(|parent| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let tile_pos = TilePos { x, y };
                let tex_idx = TileTextureIndex(0);
                let tile_entity = parent
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            texture_index: tex_idx,
                            tilemap_id,
                            ..Default::default()
                        },
                        Name::new(format!("Tile ({}, {})", x, y)),
                    ))
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    tilemap_entity.insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        map_type,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
