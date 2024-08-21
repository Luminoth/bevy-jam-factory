use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::splash::*;
use crate::resources::splash::*;
use crate::state::AppState;
use crate::ui::*;

pub fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("entering Splash state");

    commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnSplashScreen));

    let image = asset_server.load("images/splash.png");

    // TODO: fade-in / fade-out
    // TODO: multiple splash screens (PIGSquad, Bevy)

    let canvas = create_canvas(&mut commands, "Main Menu");
    commands.entity(canvas).with_children(|parent| {
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(200.0),
                ..default()
            },
            image: UiImage::new(image),
            ..default()
        });
    });

    commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

pub fn exit(mut commands: Commands) {
    info!("exiting Splash state");

    commands.remove_resource::<ClearColor>();
    commands.remove_resource::<SplashTimer>();
}

pub fn update(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut contexts: EguiContexts,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::MainMenu);
    }

    // TODO: this is just for debugging
    // while there is no actual splash
    egui::Window::new("Splash").show(contexts.ctx_mut(), |ui| {
        ui.label("Waiting for splash ...");
    });
}
