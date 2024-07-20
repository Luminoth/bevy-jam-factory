mod plugins;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins(plugins::FactoryPlugin);

    app.run();
}
