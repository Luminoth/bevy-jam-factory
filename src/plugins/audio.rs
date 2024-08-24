use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Music;

#[derive(Debug, Component)]
pub struct OneShotAudio;

#[derive(Debug, Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, _app: &mut App) {}
}
