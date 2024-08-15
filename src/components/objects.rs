use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString)]
pub enum ObjectType {
    Resources,
}

#[derive(Debug, Component)]
pub struct Object {
    #[allow(dead_code)]
    pub r#type: ObjectType,
    // TODO: need property storage
}
