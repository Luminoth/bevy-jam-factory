use bevy::prelude::*;

use crate::states::AppState;
use crate::systems::game::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_game);
    }
}
