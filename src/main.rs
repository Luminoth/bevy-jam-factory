#![deny(warnings)]

mod assets;
mod components;
mod game;
mod plugins;
mod resources;
mod state;
mod systems;
mod tiled;
mod tilemap;
mod ui;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_mod_picking::prelude::*;

#[inline]
pub fn get_world_position_from_cursor_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor_position| camera.viewport_to_world_2d(camera_transform, cursor_position))
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        // core plugins
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Jam - Factory".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            })
            // prevent blurry sprites
            .set(ImagePlugin::default_nearest()),
        //bevy::diagnostic::LogDiagnosticsPlugin::default(),
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        //bevy::render::diagnostic::RenderDiagnosticsPlugin,
        bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        // third-party plugins
        DefaultPickingPlugins,
        TilemapPlugin,
        bevy_egui::EguiPlugin,
        // inspectors
        // TODO: why does the world inspector not pick up custom resource types?
        // using register_type() on them doesn't seem to fix it
        // TODO: might have outgrown the quick plugins: https://docs.rs/bevy-inspector-egui/0.25.2/bevy_inspector_egui/#use-case-2-manual-ui
        bevy_inspector_egui::quick::WorldInspectorPlugin::default()
            .run_if(input_toggle_active(false, KeyCode::Backquote)),
        bevy_inspector_egui::quick::StateInspectorPlugin::<state::AppState>::default()
            .run_if(input_toggle_active(false, KeyCode::Backquote)),
        bevy_inspector_egui::quick::ResourceInspectorPlugin::<resources::game::Inventory>::default(
        )
        .run_if(input_toggle_active(false, KeyCode::Backquote)),
    ));

    /*app.insert_resource(bevy_egui::EguiSettings {
        scale_factor: 0.75,
        ..Default::default()
    });*/

    // TODO: move to a state init or something
    app.init_state::<state::AppState>()
        .add_sub_state::<state::IsPaused>()
        .enable_state_scoped_entities::<state::IsPaused>();

    app.add_plugins((
        plugins::TiledMapPlugin,
        plugins::TiledPickingBackend,
        plugins::UiPlugin,
        plugins::GameUiPlugin,
        plugins::SplashPlugin,
        plugins::MainMenuPlugin,
        plugins::PauseMenuPlugin,
        plugins::GamePlugin,
    ));

    // TODO: move to a plugin
    app.add_systems(
        Update,
        systems::debug::debug_ui.run_if(input_toggle_active(false, KeyCode::Backquote)),
    );

    app.run();
}
