use bevy::prelude::*;

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

#[derive(Debug, Component)]
pub struct InventoryWindow;
