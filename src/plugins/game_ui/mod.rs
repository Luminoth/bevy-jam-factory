pub mod inventory;
pub mod log;
pub mod object_info;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::plugins::{IsPaused, IsPointerCaptured};
use crate::AppState;

#[derive(Debug, Default)]
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<log::LogEvent>()
            .add_systems(OnEnter(AppState::LoadAssets), load_assets)
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    setup,
                    log::setup_window,
                    object_info::setup_window,
                    inventory::setup_window,
                ),
            )
            .add_systems(
                Update,
                (
                    log::log_event_handler,
                    object_info::update_object_info_ui
                        .run_if(object_info::should_update_object_info_ui),
                    inventory::show_inventory.run_if(input_just_pressed(KeyCode::KeyI)),
                    inventory::inventory_updated_event_handler,
                    inventory::hide_item_drag_image_event_handler,
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(OnExit(AppState::InGame), teardown);
    }
}

fn load_assets() {}

fn setup(mut commands: Commands) {
    commands.init_resource::<log::LogTextContent>();
    commands.init_resource::<IsPointerCaptured>();
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<IsPointerCaptured>();
    commands.remove_resource::<log::LogTextContent>();
}
