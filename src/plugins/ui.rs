use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;

use crate::components::ui::UiWindow;
use crate::state::{AppState, IsPaused};
use crate::systems::ui;
use crate::ui::*;

#[derive(Debug, Default, Reflect, Resource, Deref, DerefMut)]
pub struct IsPointerCaptured(pub bool);

#[derive(Debug, Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_button,));

        app.add_systems(OnEnter(AppState::LoadAssets), ui::load_assets)
            .add_systems(OnEnter(AppState::InGame), ui::setup)
            .add_systems(
                PreUpdate,
                update_pointer_capture.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    ui::update_object_info_ui.run_if(ui::should_update_object_info_ui),
                    ui::show_inventory.run_if(input_just_pressed(KeyCode::KeyI)),
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(OnExit(AppState::InGame), ui::teardown);
    }
}

#[allow(clippy::type_complexity)]
fn update_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVER.into();
            }
            Interaction::None => {
                *color = BUTTON_NORMAL.into();
            }
        }
    }
}

fn update_pointer_capture(
    mut is_pointer_captured: ResMut<IsPointerCaptured>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ui_window_query: Query<(&Node, &GlobalTransform, &ViewVisibility), With<UiWindow>>,
    mut contexts: EguiContexts,
) {
    let window = window_query.single();
    let context = contexts.ctx_mut();

    is_pointer_captured.0 = window
        .cursor_position()
        .map(|cursor_position| {
            ui_window_query
                .iter()
                .filter(|(_, _, visibility)| visibility.get())
                .any(|(node, transform, _)| {
                    let node_position = transform.translation().xy();
                    let half_size = 0.5 * node.size();
                    let min = node_position - half_size;
                    let max = node_position + half_size;
                    (min.x..max.x).contains(&cursor_position.x)
                        && (min.y..max.y).contains(&cursor_position.y)
                })
        })
        .unwrap_or_default()
        || context.is_pointer_over_area()
        || context.is_using_pointer();
}
