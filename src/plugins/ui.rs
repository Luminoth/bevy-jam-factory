use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;

use crate::state::AppState;
use crate::ui::*;

#[derive(Debug, Component)]
pub struct UiWindow;

#[derive(Debug, Component)]
pub struct UiWindowTitleBar(pub Entity);

#[derive(Debug, Component)]
pub struct UiWindowCloseButton(pub Entity);

#[derive(Debug, Component)]
pub struct UiWindowContent;

#[derive(Debug, Default, Reflect, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
}

#[derive(Debug, Default, Reflect, Resource, Deref, DerefMut)]
pub struct IsPointerCaptured(pub bool);

#[derive(Debug, Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets)
            .add_systems(
                PreUpdate,
                update_pointer_capture.run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, (update_button,));
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        font: asset_server.load(FONT),
    });
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
