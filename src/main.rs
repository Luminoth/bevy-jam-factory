mod assets;
mod components;
mod plugins;
mod states;
mod systems;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
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
        TilemapPlugin,
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backquote)),
    ));

    app.add_plugins((plugins::TiledMapPlugin, plugins::GamePlugin));

    // TODO: move to a state init or something
    app.init_state::<states::AppState>()
        .add_sub_state::<states::IsPaused>()
        .enable_state_scoped_entities::<states::IsPaused>()
        .add_plugins(
            StateInspectorPlugin::<states::AppState>::default()
                .run_if(input_toggle_active(false, KeyCode::Backquote)),
        );

    // TODO: move to main menu plugin
    app.add_systems(
        OnEnter(states::AppState::MainMenu),
        systems::main_menu::setup_main_menu,
    )
    .add_systems(
        Update,
        systems::main_menu::update_main_menu.run_if(in_state(states::AppState::MainMenu)),
    );

    // TODO: move to pause menu plugin
    app.add_systems(
        OnEnter(states::IsPaused::Paused),
        systems::pause_menu::setup_pause_menu,
    )
    .add_systems(
        Update,
        systems::pause_menu::update_pause_menu.run_if(in_state(states::IsPaused::Paused)),
    );

    app.run();
}
