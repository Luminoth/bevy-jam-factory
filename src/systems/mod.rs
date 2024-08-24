pub mod camera;
pub mod game;
pub mod input;

use bevy::prelude::*;

pub fn cleanup_state<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
