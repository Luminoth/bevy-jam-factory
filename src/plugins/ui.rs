use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::state::{AppState, IsPaused};
use crate::systems::ui;

#[derive(Debug, Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ui::button::update,));

        app.add_systems(OnEnter(AppState::LoadAssets), ui::load_assets)
            .add_systems(OnEnter(AppState::InGame), ui::setup)
            .add_systems(
                PreUpdate,
                ui::update_pointer_capture.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    ui::update_object_info_ui.run_if(ui::should_update_object_info_ui),
                    ui::show_inventory.run_if(input_just_pressed(KeyCode::KeyI)),
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(OnExit(AppState::InGame), ui::teardown);
    }
}
