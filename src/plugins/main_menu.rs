use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::cleanup_state;
use crate::plugins::UiAssets;
use crate::ui::{check_click_event, create_button, create_canvas};
use crate::AppState;

#[derive(Debug, Component)]
pub struct MainMenu;

#[derive(Debug, Component)]
pub struct OnMainMenu;

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
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    create_canvas(&mut commands, "Main Menu")
        .insert(MainMenu)
        .with_children(|parent| {
            create_button(
                parent,
                &ui_assets,
                "Start Game",
                On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>,
                     mut game_state: ResMut<NextState<AppState>>| {
                        if !check_click_event(&event, PointerButton::Primary) {
                            return;
                        }
                        game_state.set(AppState::LoadAssets);
                    },
                ),
            );

            create_button(
                parent,
                &ui_assets,
                "Exit Game",
                On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                        if !check_click_event(&event, PointerButton::Primary) {
                            return;
                        }
                        exit.send(AppExit::Success);
                    },
                ),
            );
        });
}

fn exit(mut commands: Commands) {
    info!("exiting MainMenu state");

    commands.remove_resource::<ClearColor>();
}
