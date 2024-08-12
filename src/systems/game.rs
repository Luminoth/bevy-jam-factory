use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::assets::tiled::TiledMap;
use crate::components::{tiled::TiledMapBundle, MainCamera};

// tile maps should be bigger than this
const VIEW_WIDTH: f32 = 800.0;
const VIEW_HEIGHT: f32 = 600.0;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: VIEW_WIDTH,
        height: VIEW_HEIGHT,
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
