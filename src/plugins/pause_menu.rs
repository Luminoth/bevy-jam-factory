use bevy::prelude::*;

use crate::state::IsPaused;
use crate::systems::pause_menu;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IsPaused::Paused), pause_menu::enter);
    }
}
