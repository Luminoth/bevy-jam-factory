use bevy::prelude::*;

use crate::components::game::OnInGame;
use crate::state::{AppState, IsPaused};
use crate::systems::{camera::*, cleanup_state, game, input::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), game::enter)
            .add_systems(
                Update,
                (
                    pan_camera.run_if(in_state(IsPaused::Running)),
                    mouse_button_input.run_if(in_state(IsPaused::Running)),
                ),
            )
            .add_systems(
                OnExit(AppState::InGame),
                (game::exit, cleanup_state::<OnInGame>),
            );
    }
}
