use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::cleanup_state;
use crate::ui::create_canvas;
use crate::AppState;

/// Splash screen state tag
#[derive(Debug, Component)]
pub struct OnSplashScreen;

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

#[derive(Debug, Default)]
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), enter)
            .add_systems(Update, update.run_if(in_state(AppState::Splash)))
            .add_systems(
                OnExit(AppState::Splash),
                (exit, cleanup_state::<OnSplashScreen>, cleanup_state::<Node>),
            );
    }
}

fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("entering Splash state");

    commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnSplashScreen));

    let image = asset_server.load("images/splash.png");

    // TODO: fade-in / fade-out
    // TODO: multiple splash screens (PIGSquad, Bevy)

    create_canvas(&mut commands, "Main Menu").with_children(|parent| {
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

fn exit(mut commands: Commands) {
    info!("exiting Splash state");

    commands.remove_resource::<ClearColor>();
    commands.remove_resource::<SplashTimer>();
}

fn update(
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
