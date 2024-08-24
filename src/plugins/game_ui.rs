use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::state::{AppState, IsPaused};
use crate::systems::game_ui;

#[derive(Debug, Default)]
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadAssets), game_ui::load_assets)
            .add_systems(OnEnter(AppState::InGame), game_ui::setup)
            .add_systems(
                Update,
                (
                    game_ui::update_object_info_ui.run_if(game_ui::should_update_object_info_ui),
                    game_ui::show_inventory.run_if(input_just_pressed(KeyCode::KeyI)),
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(OnExit(AppState::InGame), game_ui::teardown);
    }
}
