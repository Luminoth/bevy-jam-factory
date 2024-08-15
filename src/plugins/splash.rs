use bevy::prelude::*;

use crate::components::splash::*;
use crate::state::AppState;
use crate::systems::{cleanup_state, splash};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), splash::enter)
            .add_systems(Update, (splash::update,).run_if(in_state(AppState::Splash)))
            .add_systems(OnExit(AppState::Splash), cleanup_state::<OnSplashScreen>);
    }
}
