mod assets;
mod components;
mod plugins;
mod states;
mod systems;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

fn main() {
    let mut app = App::new();

    // core setup
    app.add_plugins((
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
        FrameTimeDiagnosticsPlugin,
        TilemapPlugin,
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backquote)),
    ));

    // TODO: add debug menu stuff that includes displaying FPS

    app.add_plugins((
        plugins::TiledMapPlugin,
        plugins::MainMenuPlugin,
        plugins::PauseMenuPlugin,
        plugins::GamePlugin,
    ));

    // TODO: move to a state init or something
    app.init_state::<states::AppState>()
        .add_sub_state::<states::IsPaused>()
        .enable_state_scoped_entities::<states::IsPaused>()
        .add_plugins(
            StateInspectorPlugin::<states::AppState>::default()
                .run_if(input_toggle_active(false, KeyCode::Backquote)),
        );

    app.run();
}
