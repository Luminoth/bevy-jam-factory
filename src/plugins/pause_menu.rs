use bevy::prelude::*;

use crate::state::IsPaused;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IsPaused::Paused), enter);
    }
}

fn enter() {
    info!("entering Paused state");
}
