use bevy::prelude::*;

use crate::states::IsPaused;
use crate::systems::pause_menu::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IsPaused::Paused), setup_pause_menu)
            .add_systems(Update, update_pause_menu.run_if(in_state(IsPaused::Paused)));
    }
}
