use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::events::ui::*;
use crate::state::{AppState, IsPaused};
use crate::systems::ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ui::button::update,));

        app.add_event::<UpdateObjectInfoUIEvent>()
            .add_systems(OnEnter(AppState::LoadAssets), ui::load_assets)
            .add_systems(OnEnter(AppState::InGame), ui::setup)
            .add_systems(
                PreUpdate,
                ui::update_pointer_capture.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    ui::show_egui_object_info.run_if(ui::have_object_info),
                    ui::update_object_info_ui_handler,
                    //ui::show_object_info.run_if(ui::have_object_info),
                    ui::show_inventory.run_if(input_just_pressed(KeyCode::KeyI)),
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(OnExit(AppState::InGame), ui::teardown);
    }
}
