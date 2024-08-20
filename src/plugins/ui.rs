use bevy::prelude::*;

use crate::state::{AppState, IsPaused};
use crate::systems::ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                ui::button_interaction,
                ui::title_bar_interaction,
                ui::close_button_interaction,
            ),
        );

        app.add_systems(OnEnter(AppState::LoadAssets), ui::load_assets)
            .add_systems(OnEnter(AppState::InGame), ui::setup)
            .add_systems(
                PreUpdate,
                ui::update_pointer_capture.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (ui::show_object_info.run_if(ui::have_object_info),)
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(OnExit(AppState::InGame), ui::teardown);
    }
}
