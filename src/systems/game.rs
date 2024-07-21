use bevy::prelude::*;

pub fn setup_game(mut commands: Commands) {
    println!("setup game");

    commands.spawn(Camera2dBundle::default());
}
