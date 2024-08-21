pub mod button;
pub mod window;

use bevy::prelude::*;

use button::*;
use window::*;

pub const FONT: &str = "fonts/FiraSans-Bold.ttf";
pub const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn create_inventory_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    window: &Window,
) {
    let content_id = create_window(commands, &asset_server, window, (400, 200), "Inventory");
    commands.entity(content_id).with_children(|parent| {
        create_button(parent, &asset_server, "Button");
    });
}
