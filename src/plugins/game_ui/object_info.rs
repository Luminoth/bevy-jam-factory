use bevy::{prelude::*, window::PrimaryWindow};

use crate::data::objects::ObjectData;
use crate::plugins::{objects::Object, ObjectInfo, UiAssets};
use crate::ui::*;

#[derive(Debug, Component)]
pub struct ObjectInfoWindow;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectInfoData {
    ObjectId,
    ObjectType,
}

#[derive(Debug, Component)]
pub struct ObjectInfoDataUI(pub ObjectInfoData);

#[derive(Debug, Component)]
pub struct ObjectInfoResources;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectInfoResourcesData {
    ResourceType,
    Amount(u32),
}

#[derive(Debug, Component)]
pub struct ObjectInfoResourcesDataUI(pub ObjectInfoResourcesData);

pub(super) fn setup_window(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let content_id = create_window(
        &mut commands,
        &ui_assets,
        window,
        (400, 200),
        "Object Info",
        false,
        ObjectInfoWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        create_row_container(parent).with_children(|parent| {
            create_label(parent, &ui_assets, "Object ID:", 14.0, FONT_COLOR);
            create_label(parent, &ui_assets, "N/A", 14.0, FONT_COLOR)
                .insert(ObjectInfoDataUI(ObjectInfoData::ObjectId));
        });

        create_row_container(parent).with_children(|parent| {
            create_label(parent, &ui_assets, "Object Type:", 14.0, FONT_COLOR);
            create_label(parent, &ui_assets, "N/A", 14.0, FONT_COLOR)
                .insert(ObjectInfoDataUI(ObjectInfoData::ObjectType));
        });

        // Resources
        create_column_container(parent)
            .insert((
                Visibility::Hidden,
                Name::new("Resources"),
                ObjectInfoResources,
            ))
            .with_children(|parent| {
                create_row_container(parent).with_children(|parent| {
                    create_label(parent, &ui_assets, "Resource Type:", 14.0, FONT_COLOR);
                    create_label(parent, &ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                        ObjectInfoResourcesDataUI(ObjectInfoResourcesData::ResourceType),
                    );
                });

                create_row_container(parent).with_children(|parent| {
                    create_label(parent, &ui_assets, "Amount:", 14.0, FONT_COLOR);
                    create_label(parent, &ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                        ObjectInfoResourcesDataUI(ObjectInfoResourcesData::Amount(0)),
                    );
                });
            });
    });
}

pub(super) fn should_update_object_info_ui(
    object: Option<Res<ObjectInfo>>,
    window_query: Query<&ViewVisibility, With<ObjectInfoWindow>>,
) -> bool {
    let window_visible = window_query
        .get_single()
        .map(|visible| visible.get())
        .unwrap_or_default();

    object.is_some() && window_visible
}

// TODO: this running constantly while the UI is shown isn't great
// we should use an event or something to just update it when something changes
// (or update it once entirely on show, and then only update dynamic stuff while open)
#[allow(clippy::type_complexity)]
pub(super) fn update_object_info_ui(
    object: Res<ObjectInfo>,
    object_query: Query<&Object>,
    mut text_set: ParamSet<(
        Query<(&mut Text, &ObjectInfoDataUI)>,
        Query<(&mut Text, &mut ObjectInfoResourcesDataUI)>,
    )>,
    mut resources_section_query: Query<&mut Visibility, With<ObjectInfoResources>>,
) {
    let object = object_query
        .get(object.0)
        .expect("Object tile missing Object!");

    for (mut text, data) in text_set.p0().iter_mut() {
        match data.0 {
            ObjectInfoData::ObjectId => {
                // TODO: only update if changed
                text.sections.get_mut(0).unwrap().value = object.get_id().to_string();
            }
            ObjectInfoData::ObjectType => {
                // TODO: only update if changed
                text.sections.get_mut(0).unwrap().value = object.get_type().to_string();
            }
        }
    }

    match &object.0 {
        ObjectData::Resources { r#type, amount, .. } => {
            for (mut text, mut data) in text_set.p1().iter_mut() {
                match &mut data.0 {
                    ObjectInfoResourcesData::ResourceType => {
                        // TODO: only update if changed
                        text.sections.get_mut(0).unwrap().value = r#type.to_string();
                    }
                    ObjectInfoResourcesData::Amount(prev) => {
                        if *prev != *amount {
                            text.sections.get_mut(0).unwrap().value = amount.to_string();
                            *prev = *amount;
                        }
                    }
                }
            }

            let mut visibility = resources_section_query.single_mut();
            *visibility = Visibility::Inherited;
        }
    }
}
