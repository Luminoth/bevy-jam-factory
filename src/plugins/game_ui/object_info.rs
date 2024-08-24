use bevy::prelude::*;

use crate::data::objects::ObjectData;
use crate::plugins::{objects::Object, ObjectInfo};

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
    Amount,
}

#[derive(Debug, Component)]
pub struct ObjectInfoResourcesDataUI(pub ObjectInfoResourcesData);

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

#[allow(clippy::type_complexity)]
pub(super) fn update_object_info_ui(
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
