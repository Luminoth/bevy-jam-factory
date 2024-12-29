use bevy::prelude::*;

use crate::cleanup_state;
use crate::plugins::ui::UiAssets;
use crate::ui::{create_button, create_canvas};
use crate::AppState;

/// Main menu state tag
#[derive(Debug, Component)]
pub struct OnMainMenu;

/// Main menu canvas tag
#[derive(Debug, Component)]
pub struct MainMenuCanvas;

#[derive(Debug, Default)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), enter)
            .add_systems(
                OnExit(AppState::MainMenu),
                (exit, cleanup_state::<OnMainMenu>, cleanup_state::<Node>),
            );
    }
}

fn enter(mut commands: Commands, ui_assets: Res<UiAssets>) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2d, OnMainMenu));

    create_canvas(&mut commands, "Main Menu")
        .insert(MainMenuCanvas)
        .with_children(|parent| {
            create_button(parent, &ui_assets, "Start Game").observe(
                |event: Trigger<Pointer<Click>>, mut game_state: ResMut<NextState<AppState>>| {
                    if event.button == PointerButton::Primary {
                        game_state.set(AppState::LoadAssets);
                    }
                },
            );

            create_button(parent, &ui_assets, "Exit Game").observe(
                |event: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                    if event.button == PointerButton::Primary {
                        exit.send(AppExit::Success);
                    }
                },
            );
        });
}

fn exit(mut commands: Commands) {
    info!("exiting MainMenu state");

    commands.remove_resource::<ClearColor>();
}
