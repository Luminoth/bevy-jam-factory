pub mod button;
pub mod window;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::components::ui::*;
use button::*;
use window::*;

pub const FONT: &str = "fonts/FiraSans-Bold.ttf";
pub const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn create_canvas(commands: &mut Commands, name: impl Into<String>) -> Entity {
    commands
        .spawn((
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
            Name::new(name.into()),
        ))
        .id()
}

pub fn create_object_info_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window: &Window,
) {
    let content_id = create_window(
        commands,
        asset_server,
        window,
        (400, 200),
        "Object Info",
        false,
        ObjectInfoWindow,
    );
    commands.entity(content_id).with_children(|_parent| {
        // TODO:
    });
}

pub fn create_inventory_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window: &Window,
) {
    let content_id = create_window(
        commands,
        asset_server,
        window,
        (400, 200),
        "Inventory",
        false,
        InventoryWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        create_button(
            parent,
            asset_server,
            "Button",
            On::<Pointer<Click>>::run(move || info!("Button clicked!")),
        );
    });
}
