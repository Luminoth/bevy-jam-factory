use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::plugins::{BackgroundMusic, IsPaused, UiAssets};
use crate::ui::{create_button, create_canvas};
use crate::AppState;

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
            create_button(
                parent,
                &ui_assets,
                "Resume Game",
                On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>,
                     mut pause_state: ResMut<NextState<IsPaused>>| {
                        if event.target != event.listener() {
                            return;
                        }
                        if event.button != PointerButton::Primary {
                            return;
                        }

                        pause_state.set(IsPaused::Running);
                    },
                ),
            );

            create_button(
                parent,
                &ui_assets,
                "Quit Game",
                On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>,
                     mut game_state: ResMut<NextState<AppState>>| {
                        if event.target != event.listener() {
                            return;
                        }
                        if event.button != PointerButton::Primary {
                            return;
                        }

                        game_state.set(AppState::MainMenu);
                    },
                ),
            );
        });
}

fn enter(
    mut window_query: Query<&mut Visibility, With<PauseMenu>>,
    mut _music_query: Query<&mut PlaybackSettings, With<BackgroundMusic>>,
) {
    info!("entering Paused state");

    *window_query.single_mut() = Visibility::Visible;

    // TODO: duck the music
}

fn exit(
    mut window_query: Query<&mut Visibility, With<PauseMenu>>,
    mut _music_query: Query<&mut PlaybackSettings, With<BackgroundMusic>>,
) {
    info!("exiting Paused state");

    *window_query.single_mut() = Visibility::Hidden;

    // TODO: restore the music
}
