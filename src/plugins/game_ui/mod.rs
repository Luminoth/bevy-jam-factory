pub mod inventory;
pub mod object_info;

use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;

use crate::plugins::{IsPaused, IsPointerCaptured, UiAssets};
use crate::ui::*;
use crate::AppState;

#[derive(Debug, Component)]
pub struct LogWindow;

#[derive(Debug, Component)]
pub struct LogText;

#[derive(Debug, Default, Reflect, Resource)]
pub struct LogTextContent(String);

// TODO: finish up being able to log stuff
// (log when music is toggled as an example)
// also need to be able to interact below the log window
// (eg. fully ignore picking with it)

#[derive(Debug, Default)]
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadAssets), load_assets)
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (
                    object_info::update_object_info_ui
                        .run_if(object_info::should_update_object_info_ui),
                    inventory::show_inventory.run_if(input_just_pressed(KeyCode::KeyI)),
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(OnExit(AppState::InGame), teardown);
    }
}

fn load_assets() {}

fn setup(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    commands.init_resource::<LogTextContent>();

    let log_id = create_fixed_window(
        &mut commands,
        ((window.width() - 400.0) as usize, 0),
        (400, 200),
        "Log",
        true,
        LogWindow,
    );
    commands.entity(log_id).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 12.0,
                    color: FONT_COLOR,
                },
            ),
            Name::new("Log"),
            Pickable::IGNORE,
            LogText,
        ));
    });

    create_object_info_ui(&mut commands, &ui_assets, window);
    create_inventory_ui(&mut commands, &ui_assets, window);

    commands.init_resource::<IsPointerCaptured>();
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<IsPointerCaptured>();
}
