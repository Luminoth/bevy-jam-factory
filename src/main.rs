mod plugins;
mod states;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.init_state::<states::AppState>()
        .add_sub_state::<states::IsPaused>()
        .add_plugins(plugins::FactoryPlugin);

    app.run();
}
