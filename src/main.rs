#![deny(warnings)]

mod assets;
mod components;
mod plugins;
mod resources;
mod state;
mod systems;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecs_tilemap::prelude::*;

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
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        // third-party plugins
        TilemapPlugin,
        bevy_egui::EguiPlugin,
        // inspectors
        bevy_inspector_egui::quick::WorldInspectorPlugin::new()
            .run_if(input_toggle_active(false, KeyCode::Backquote)),
        bevy_inspector_egui::quick::StateInspectorPlugin::<state::AppState>::default()
            .run_if(input_toggle_active(false, KeyCode::Backquote)),
    ));

    // TODO: move to a state init or something
    app.init_state::<state::AppState>()
        .add_sub_state::<state::IsPaused>()
        .enable_state_scoped_entities::<state::IsPaused>();

    // TODO: add debug menu stuff that includes displaying FPS

    app.add_plugins((
        plugins::TiledMapPlugin,
        plugins::SplashPlugin,
        plugins::MainMenuPlugin,
        plugins::PauseMenuPlugin,
        plugins::GamePlugin,
    ));

    app.run();
}
