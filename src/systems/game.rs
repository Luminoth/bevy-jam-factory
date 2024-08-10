use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::CursorGrabMode;

use crate::assets::tiled::TiledMap;
use crate::components::tiled::TiledMapBundle;

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: 800.0,
        height: 600.0,
    };
    commands.spawn((camera, Name::new("Main Camera")));

    let map_handle: Handle<TiledMap> = asset_server.load("map.tmx");

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });

    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}
