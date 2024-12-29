use bevy::prelude::*;

use crate::plugins::{audio::Music, game::IsPaused, ui::UiAssets};
use crate::ui::{create_button, create_canvas};
use crate::AppState;

/// Pause menu state tag
#[derive(Debug, Component)]
pub struct PauseMenu;

#[derive(Debug, Default)]
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnEnter(IsPaused::Paused), enter)
            .add_systems(OnEnter(IsPaused::Running), exit);
    }
}

fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    // TODO: this canvas should be transparent grey
    create_canvas(&mut commands, "Pause Menu")
        .insert(PauseMenu)
        .with_children(|parent| {
            create_button(parent, &ui_assets, "Resume Game").observe(
                |event: Trigger<Pointer<Click>>, mut pause_state: ResMut<NextState<IsPaused>>| {
                    if event.button == PointerButton::Primary {
                        pause_state.set(IsPaused::Running);
                    }
                },
            );

            create_button(parent, &ui_assets, "Quit Game").observe(
                |event: Trigger<Pointer<Click>>, mut game_state: ResMut<NextState<AppState>>| {
                    if event.button == PointerButton::Primary {
                        game_state.set(AppState::MainMenu);
                    }
                },
            );
        });
}

fn enter(
    mut window_query: Query<&mut Visibility, With<PauseMenu>>,
    mut _music_query: Query<&mut PlaybackSettings, With<Music>>,
) {
    info!("entering Paused state");

    *window_query.single_mut() = Visibility::Visible;

    // TODO: duck the music
}

fn exit(
    mut window_query: Query<&mut Visibility, With<PauseMenu>>,
    mut _music_query: Query<&mut PlaybackSettings, With<Music>>,
) {
    info!("exiting Paused state");

    *window_query.single_mut() = Visibility::Hidden;

    // TODO: restore the music
}
