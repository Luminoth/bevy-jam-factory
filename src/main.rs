mod plugins;
mod states;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Jam - Factory".into(),
            resolution: (1280.0, 720.0).into(),
            ..default()
        }),
        ..default()
    }));

    app.init_state::<states::AppState>()
        .add_sub_state::<states::IsPaused>()
        .add_plugins(plugins::FactoryPlugin);

    app.run();
}
