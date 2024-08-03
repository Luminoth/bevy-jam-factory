use bevy::prelude::*;

use crate::assets::tiled::TiledMap;
use crate::components::tiled::TiledMapBundle;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), Name::new("Main Camera")));

    let map_handle: Handle<TiledMap> = asset_server.load("map.tmx");

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
