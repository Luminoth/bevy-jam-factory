use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::state::AppState;
use crate::systems::cleanup_state;
use crate::ui::{create_button, create_canvas};

#[derive(Debug, Component)]
pub struct OnMainMenu;

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

fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    create_canvas(&mut commands, "Main Menu").with_children(|parent| {
        create_button(
            parent,
            &asset_server,
            "Start Game",
            On::<Pointer<Click>>::run(
                |event: Listener<Pointer<Click>>, mut game_state: ResMut<NextState<AppState>>| {
                    if event.target != event.listener() {
                        return;
                    }
                    if event.button != PointerButton::Primary {
                        return;
                    }

                    game_state.set(AppState::LoadAssets);
                },
            ),
        );

        create_button(
            parent,
            &asset_server,
            "Quit Game",
            On::<Pointer<Click>>::run(
                |event: Listener<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                    if event.target != event.listener() {
                        return;
                    }
                    if event.button != PointerButton::Primary {
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
