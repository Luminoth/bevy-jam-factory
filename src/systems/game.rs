use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};

use bevy_egui::{egui, EguiContexts};

use crate::assets::tiled::TiledMap;
use crate::components::{
    game::OnInGame,
    tiled::{TiledMapBundle, TiledMapTileLayer},
    MainCamera,
};
use crate::state::AppState;

// tile maps should be bigger than this
const VIEW_WIDTH: f32 = 800.0;
const VIEW_HEIGHT: f32 = 600.0;

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("map.tmx");

    commands.spawn((
        TiledMapBundle {
            tiled_map: map_handle,
            ..Default::default()
        },
        Name::new("Tiled Map"),
        OnInGame,
    ));

    info!("Waiting for assets ...");
}

pub fn wait_for_assets(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<AppState>>,
    tiledmap_query: Query<&TiledMapTileLayer>,
) {
    egui::Window::new("Loading").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.label("Loading assets ...");
        });
    });

    if tiledmap_query.is_empty() {
        return;
    }

    info!("Assets loaded, starting game ...");
    game_state.set(AppState::InGame);
}

pub fn enter(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    info!("entering InGame state");

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: VIEW_WIDTH,
        height: VIEW_HEIGHT,
    };
    commands.spawn((
        camera_bundle,
        Name::new("Main Camera"),
        MainCamera,
        OnInGame,
    ));

    // center the cursor so the camera doesn't start panning immediately
    let mut window = window_query.single_mut();
    let center_cursor_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    window.set_cursor_position(Some(center_cursor_pos));
}

pub fn exit() {
    info!("exiting InGame state");
}
