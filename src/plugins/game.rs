use bevy::{input::common_conditions::*, prelude::*};

use crate::components::game::OnInGame;
use crate::state::{AppState, IsPaused};
use crate::systems::{camera, cleanup_state, game, input};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadAssets), game::load_assets)
            .add_systems(
                Update,
                (game::wait_for_assets,).run_if(in_state(AppState::LoadAssets)),
            )
            .add_systems(OnEnter(AppState::InGame), game::enter)
            .add_systems(
                Update,
                (
                    camera::pan,
                    input::tile_info.run_if(input_just_released(MouseButton::Right)),
                    input::start_drag.run_if(input_just_pressed(MouseButton::Left)),
                    input::stop_drag.run_if(input_just_released(MouseButton::Left)),
                    // TODO: instead of "just_pressed" we should check for a Drag resource existing
                    // (eg. resource_exists::<DragOperation>)
                    input::drag.run_if(input_pressed(MouseButton::Left)),
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(
                OnExit(AppState::InGame),
                (game::exit, cleanup_state::<OnInGame>),
            );
    }
}
