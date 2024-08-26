use std::str::FromStr;

use bevy::prelude::*;

use crate::tiled::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumString, strum::Display, Reflect)]
pub enum ResourceType {
    Iron,
}

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::Display)]
pub enum ObjectType {
    Resources,
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum ObjectData {
    Resources {
        id: u32,
        r#type: ResourceType,
        amount: u32,
    },
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

                Ok(Self::Resources {
                    id: object.id(),
                    r#type: resource_type,
                    amount: amount as u32,
                })
            }
        }
    }

    pub fn get_id(&self) -> u32 {
        match self {
            Self::Resources { id, .. } => *id,
        }
    }

    pub fn get_type(&self) -> ObjectType {
        match self {
            Self::Resources { .. } => ObjectType::Resources,
        }
    }
}
