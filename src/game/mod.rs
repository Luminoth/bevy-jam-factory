use std::str::FromStr;

use bevy_egui::egui::Ui;

use crate::tiled::*;

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::Display)]
pub enum ResourceType {
    Iron,
}

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::Display)]
pub enum ObjectType {
    Resources,
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum ObjectData {
    Resources(ResourceType, u32),
}

impl ObjectData {
    pub fn new(layer_id: u32, object: &tiled::Object) -> anyhow::Result<Self> {
        let Ok(r#type) = ObjectType::from_str(&object.user_type) else {
            anyhow::bail!(
                "Object layer {} has invalid class {} for object {}",
                layer_id,
                object.user_type,
                object.id(),
            )
        };

        match r#type {
            ObjectType::Resources => {
                let resource_type = require_object_string_property(object, "ResourceType")?;
                let Ok(resource_type) = ResourceType::from_str(resource_type) else {
                    anyhow::bail!(
                        "Resource {} has invalid ResourceType {:?}",
                        object.id(),
                        resource_type,
                    )
                };

                let amount = require_object_int_property(object, "Amount")?.max(0);

                Ok(Self::Resources(resource_type, amount as u32))
            }
        }
    }

    pub fn get_type(&self) -> ObjectType {
        match self {
            Self::Resources(..) => ObjectType::Resources,
        }
    }

    pub fn show_info(&self, ui: &mut Ui) {
        match self {
            Self::Resources(r#type, amount) => {
                ui.label(format!("Resource Type: {}", r#type));
                ui.label(format!("Amount: {}", amount));
            }
        }
    }
}
