pub mod camera;
pub mod debug;
pub mod game;
pub mod input;
pub mod pause_menu;
pub mod splash;
pub mod tiled;
pub mod ui;

use bevy::prelude::*;

pub fn cleanup_state<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
