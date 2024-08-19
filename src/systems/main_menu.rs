use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::main_menu::*;
use crate::state::AppState;

pub fn enter(mut commands: Commands) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));
}

pub fn exit(mut commands: Commands) {
    info!("exiting MainMenu state");

    commands.remove_resource::<ClearColor>();
}

pub fn update(
    mut exit: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<AppState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Main Menu").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            if ui.button("Start Game").clicked() {
                game_state.set(AppState::LoadAssets);
            }

            if ui.button("Quit Game").clicked() {
                exit.send(AppExit::Success);
            }
        });
    });
}
