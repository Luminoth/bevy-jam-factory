pub mod inventory;
pub mod object_info;

use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};

use crate::plugins::{IsPaused, IsPointerCaptured, UiAssets};
use crate::ui::*;
use crate::AppState;

#[derive(Debug, Default)]
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadAssets), load_assets)
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (
                    object_info::update_object_info_ui
                        .run_if(object_info::should_update_object_info_ui),
                    inventory::show_inventory.run_if(input_just_pressed(KeyCode::KeyI)),
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(OnExit(AppState::InGame), teardown);
    }
}

fn load_assets() {}

fn setup(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    create_object_info_ui(&mut commands, &ui_assets, window);
    create_inventory_ui(&mut commands, &ui_assets, window);

    commands.init_resource::<IsPointerCaptured>();
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<IsPointerCaptured>();
}
