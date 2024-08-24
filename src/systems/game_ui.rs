use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{game_ui::*, objects::Object};
use crate::data::objects::ObjectData;
use crate::plugins::{IsPointerCaptured, UiAssets};
use crate::resources::game::ObjectInfo;
use crate::ui::*;

pub fn should_update_object_info_ui(
    object: Option<Res<ObjectInfo>>,
    window_query: Query<&ViewVisibility, With<ObjectInfoWindow>>,
) -> bool {
    let window_visible = window_query
        .get_single()
        .map(|visible| visible.get())
        .unwrap_or_default();

    object.is_some() && window_visible
}

pub fn load_assets() {}

pub fn setup(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    create_object_info_ui(&mut commands, &ui_assets, window);
    create_inventory_ui(&mut commands, &ui_assets, window);

    commands.init_resource::<IsPointerCaptured>();
}

pub fn teardown(mut commands: Commands) {
    commands.remove_resource::<IsPointerCaptured>();
}

#[allow(clippy::type_complexity)]
pub fn update_object_info_ui(
    object: Res<ObjectInfo>,
    object_query: Query<&Object>,
    mut text_set: ParamSet<(
        Query<(&mut Text, &ObjectInfoDataUI)>,
        Query<(&mut Text, &ObjectInfoResourcesDataUI)>,
    )>,
    mut resources_section_query: Query<&mut Visibility, With<ObjectInfoResources>>,
) {
    let object = object_query
        .get(object.0)
        .expect("Object tile missing Object!");

    // TODO: we should only update the fields that have changed

    for (mut text, data) in text_set.p0().iter_mut() {
        match data.0 {
            ObjectInfoData::ObjectId => {
                text.sections.get_mut(0).unwrap().value = format!("{}", object.get_id());
            }
            ObjectInfoData::ObjectType => {
                text.sections.get_mut(0).unwrap().value = format!("{}", object.get_type());
            }
        }
    }

    match &object.0 {
        ObjectData::Resources { r#type, amount, .. } => {
            for (mut text, data) in text_set.p1().iter_mut() {
                match data.0 {
                    ObjectInfoResourcesData::ResourceType => {
                        text.sections.get_mut(0).unwrap().value = format!("{}", r#type);
                    }
                    ObjectInfoResourcesData::Amount => {
                        text.sections.get_mut(0).unwrap().value = format!("{}", amount);
                    }
                }
            }

            let mut visibility = resources_section_query.single_mut();
            *visibility = Visibility::Inherited;
        }
    }
}

pub fn show_inventory(mut window_query: Query<&mut Visibility, With<InventoryWindow>>) {
    *window_query.single_mut() = Visibility::Visible;
}
