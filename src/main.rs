#![deny(warnings)]

mod assets;
mod audio;
mod data;
mod plugins;
mod tiled;
mod tilemap;
mod ui;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_simple_scroll_view::ScrollViewPlugin;
use bevy_tweening::TweeningPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum AppState {
    //#[default]
    Splash,
    #[default]
    MainMenu,
    LoadAssets,
    InGame,
}

#[inline]
pub fn get_world_position_from_cursor_position(
    cursor_position: Option<Vec2>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    cursor_position
        .and_then(|cursor_position| camera.viewport_to_world_2d(camera_transform, cursor_position))
}

pub fn cleanup_state<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

const DEFAULT_RESOLUTION: (f32, f32) = (1280.0, 720.0);

fn main() {
    let mut app = App::new();

    // core plugins
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Funemployment (Bevy) Jam - Factory".into(),
                    resolution: DEFAULT_RESOLUTION.into(),
                    ..default()
                }),
                ..default()
            })
            .set(bevy::log::LogPlugin {
                // default bevy filter plus silence some spammy 3rd party crates
                filter:
                    "wgpu=error,naga=warn,symphonia_core=error,symphonia_bundle_mp3=error,bevy_simple_scroll_view=error"
                        .to_string(),
                ..default()
            })
            // prevent blurry sprites
            .set(ImagePlugin::default_nearest()));

    // third-party plugins
    app.add_plugins((
        DefaultPickingPlugins,
        TilemapPlugin,
        ScrollViewPlugin,
        TweeningPlugin,
        bevy_egui::EguiPlugin,
    ));

    /*app.insert_resource(bevy_egui::EguiSettings {
        scale_factor: 0.75,
        ..Default::default()
    });*/

    app.init_state::<AppState>();

    // game plugins
    app.add_plugins((
        plugins::TiledMapPlugin,
        plugins::TiledPickingBackend,
        plugins::UiPlugin,
        plugins::AudioPlugin,
        plugins::GameUiPlugin,
        plugins::SplashPlugin,
        plugins::MainMenuPlugin,
        plugins::PauseMenuPlugin,
        plugins::GamePlugin,
        plugins::DebugPlugin,
    ));

    app.run();
}
