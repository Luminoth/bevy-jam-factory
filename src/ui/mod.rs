mod button;
mod label;
mod window;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::components::game_ui;
use crate::plugins::UiAssets;
pub use button::*;
pub use label::*;
pub use window::*;

pub const FONT: &str = "fonts/FiraSans-Bold.ttf";
pub const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn create_canvas<'a>(commands: &'a mut Commands, name: impl AsRef<str>) -> EntityCommands<'a> {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        Name::new(format!("Ui Canvas - {}", name.as_ref())),
        Pickable::IGNORE,
    ))
}

pub fn create_column_container<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        Name::new("Column"),
        Pickable::IGNORE,
    ))
}

pub fn create_row_container<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        Name::new("Row"),
        Pickable::IGNORE,
    ))
}

pub fn create_object_info_ui(commands: &mut Commands, ui_assets: &Res<UiAssets>, window: &Window) {
    let content_id = create_window(
        commands,
        ui_assets,
        window,
        (400, 200),
        "Object Info",
        false,
        game_ui::ObjectInfoWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        create_row_container(parent).with_children(|parent| {
            create_label(parent, ui_assets, "Object ID:", 14.0, FONT_COLOR);
            create_label(parent, ui_assets, "N/A", 14.0, FONT_COLOR)
                .insert(game_ui::ObjectInfoDataUI(game_ui::ObjectInfoData::ObjectId));
        });

        create_row_container(parent).with_children(|parent| {
            create_label(parent, ui_assets, "Object Type:", 14.0, FONT_COLOR);
            create_label(parent, ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                game_ui::ObjectInfoDataUI(game_ui::ObjectInfoData::ObjectType),
            );
        });

        // Resources
        create_column_container(parent)
            .insert((
                Visibility::Hidden,
                Name::new("Resources"),
                game_ui::ObjectInfoResources,
            ))
            .with_children(|parent| {
                create_row_container(parent).with_children(|parent| {
                    create_label(parent, ui_assets, "Resource Type:", 14.0, FONT_COLOR);
                    create_label(parent, ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                        game_ui::ObjectInfoResourcesDataUI(
                            game_ui::ObjectInfoResourcesData::ResourceType,
                        ),
                    );
                });

                create_row_container(parent).with_children(|parent| {
                    create_label(parent, ui_assets, "Amount:", 14.0, FONT_COLOR);
                    create_label(parent, ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                        game_ui::ObjectInfoResourcesDataUI(
                            game_ui::ObjectInfoResourcesData::Amount,
                        ),
                    );
                });
            });
    });
}

pub fn create_inventory_ui(commands: &mut Commands, ui_assets: &Res<UiAssets>, window: &Window) {
    let content_id = create_window(
        commands,
        ui_assets,
        window,
        (400, 200),
        "Inventory",
        false,
        game_ui::InventoryWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        create_button(
            parent,
            ui_assets,
            "Button",
            On::<Pointer<Click>>::run(move || info!("Button clicked!")),
        );
    });
}
