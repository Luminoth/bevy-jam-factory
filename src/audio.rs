use bevy::{audio::Volume, prelude::*};

use crate::plugins::*;

pub fn start_music(commands: &mut Commands, source: Handle<AudioSource>) {
    commands.spawn((
        AudioBundle {
            source,
            settings: PlaybackSettings {
                volume: Volume::new(0.25),
                ..PlaybackSettings::LOOP
            },
        },
        Name::new("Music"),
        Music,
        OnInGame,
    ));
}

#[allow(dead_code)]
pub fn stop_music(commands: &mut Commands, music_query: &Query<Entity, With<Music>>) {
    for music in music_query {
        commands.entity(music).despawn();
    }
}

#[allow(dead_code)]
pub fn play_oneshot_audio(commands: &mut Commands, source: Handle<AudioSource>) {
    commands.spawn((
        AudioBundle {
            source,
            settings: PlaybackSettings {
                volume: Volume::new(0.5),
                ..PlaybackSettings::DESPAWN
            },
        },
        Name::new("OneShot Audio"),
        OneShotAudio,
        OnInGame,
    ));
}
