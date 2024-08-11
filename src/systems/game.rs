use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::assets::tiled::TiledMap;
use crate::components::{tiled::TiledMapBundle, MainCamera};

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: 800.0,
        height: 600.0,
    };
    commands.spawn((camera_bundle, Name::new("Main Camera"), MainCamera));

    let map_handle: Handle<TiledMap> = asset_server.load("map.tmx");

    commands.spawn((
        TiledMapBundle {
            tiled_map: map_handle,
            ..Default::default()
        },
        Name::new("Tiled Map"),
    ));
}
