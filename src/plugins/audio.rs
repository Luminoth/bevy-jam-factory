use bevy::{input::common_conditions::input_just_released, prelude::*};

use crate::audio::*;

#[derive(Debug, Component)]
pub struct Music;

#[derive(Debug, Component)]
pub struct OneShotAudio;

#[derive(Debug, Default, Reflect, Resource)]
pub struct AudioAssets {
    pub music: Handle<AudioSource>,
}

impl AudioAssets {
    pub fn is_loaded(&self, audio_assets: &Res<Assets<AudioSource>>) -> bool {
        audio_assets.contains(&self.music)
    }
}

#[derive(Debug, Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_music.run_if(input_just_released(KeyCode::KeyM)),
        );
    }
}

fn toggle_music(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    music_query: Query<Entity, With<Music>>,
) {
    if music_query.is_empty() {
        start_music(&mut commands, audio_assets.music.clone());
    } else {
        stop_music(&mut commands, &music_query);
    }
}
