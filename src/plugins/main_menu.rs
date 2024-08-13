use bevy::prelude::*;

use crate::states::AppState;
use crate::systems::main_menu::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                update_main_menu.run_if(in_state(AppState::MainMenu)),
            );
    }
}
