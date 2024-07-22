mod plugins;
mod states;
mod systems;
mod tiled;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

fn main() {
    let mut app = App::new();

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
        tiled::TiledMapPlugin,
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backquote)),
        StateInspectorPlugin::<states::AppState>::default()
            .run_if(input_toggle_active(false, KeyCode::Backquote)),
    ));

    app.init_state::<states::AppState>()
        .add_sub_state::<states::IsPaused>()
        .add_systems(
            OnEnter(states::AppState::MainMenu),
            systems::setup_main_menu,
        )
        .add_systems(
            Update,
            systems::update_main_menu.run_if(in_state(states::AppState::MainMenu)),
        )
        .add_systems(OnEnter(states::AppState::InGame), systems::setup_game)
        .add_systems(OnEnter(states::IsPaused::Paused), systems::setup_pause_menu)
        .enable_state_scoped_entities::<states::IsPaused>()
        .add_plugins(plugins::FactoryPlugin);

    app.run();
}
