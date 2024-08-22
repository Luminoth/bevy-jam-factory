use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::components::main_menu::*;
use crate::state::AppState;
use crate::ui::{button::*, *};

pub fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn exit(mut commands: Commands) {
    info!("exiting MainMenu state");

    commands.remove_resource::<ClearColor>();
}
