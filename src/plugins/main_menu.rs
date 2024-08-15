use bevy::prelude::*;

use crate::components::main_menu::OnMainMenu;
use crate::state::AppState;
use crate::systems::{cleanup_state, main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu::enter)
            .add_systems(
                Update,
                (main_menu::update,).run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_state::<OnMainMenu>);
    }
}
