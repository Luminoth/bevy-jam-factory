use bevy::prelude::*;

use crate::states::AppState;
use crate::states::IsPaused;
use crate::systems::camera::*;
use crate::systems::game::*;
use crate::systems::input::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_game)
            .add_systems(
                Update,
                (
                    pan_camera.run_if(in_state(IsPaused::Running)),
                    mouse_button_input.run_if(in_state(IsPaused::Running)),
                ),
            );
    }
}
