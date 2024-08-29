use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;
use bevy_simple_scroll_view::{ScrollView, ScrollableContent};

use crate::data::{items::ItemType, resources::ResourceType};
use crate::plugins::{
    game::items::{ItemDragEvent, ItemDropEvent},
    Inventory, InventoryUpdatedEvent, UiAssets,
};
use crate::ui::*;

#[derive(Debug, Component)]
pub struct InventoryWindow;

#[derive(Debug, Component)]
pub struct InventoryContent;

#[derive(Debug, Component)]
pub struct InventoryResourcesUI(pub ResourceType);

#[allow(dead_code)]
#[derive(Debug, Component)]
pub struct InventoryResourcesAmountUI(pub ResourceType, pub u32);

#[derive(Debug, Component)]
pub struct InventoryItemUI(pub ItemType);

#[allow(dead_code)]
#[derive(Debug, Component)]
pub struct InventoryItemAmountUI(pub ItemType, pub u32, pub Entity);

#[derive(Debug, Component)]
pub struct InventoryItemImage {
    pub item_type: ItemType,
}

#[derive(Debug, Default, Component)]
pub struct InventoryDragImage {
    pub item_type: Option<ItemType>,
}

#[allow(clippy::type_complexity)]
fn start_drag_inventory_item(
    event: Listener<Pointer<DragStart>>,
    item_image_query: Query<(&GlobalTransform, &InventoryItemImage)>,
    mut drag_image_query: Query<(&mut Visibility, &mut Style, &mut InventoryDragImage)>,
) {
    if !check_drag_start_event(
        event.listener(),
        event.target,
        event.button,
        PointerButton::Primary,
    ) {
        return;
    }

    let (item_image_transform, item_image) = item_image_query.get(event.target).unwrap();

    let (mut drag_image_visibility, mut drag_image_style, mut drag_image_item_type) =
        drag_image_query.single_mut();
    *drag_image_visibility = Visibility::Visible;
    drag_image_item_type.item_type = Some(item_image.item_type);

    let half_width = if let Val::Px(width) = drag_image_style.width {
        width / 2.0
    } else {
        0.0
    };
    let half_height = if let Val::Px(height) = drag_image_style.height {
        height / 2.0
    } else {
        0.0
    };

    if let Val::Px(left) = &mut drag_image_style.left {
        *left = item_image_transform.translation().x - half_width;
    }

    if let Val::Px(top) = &mut drag_image_style.top {
        *top = item_image_transform.translation().y - half_height;
    }
}

fn drag_inventory_item(
    event: Listener<Pointer<Drag>>,
    mut item_drag_events: EventWriter<ItemDragEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut drag_image_query: Query<(&mut Style, &InventoryDragImage)>,
) {
    if !check_drag_event(
        event.listener(),
        event.target,
        event.button,
        PointerButton::Primary,
    ) {
        return;
    }

    let (mut drag_image_style, drag_image_item_type) = drag_image_query.single_mut();

    if let Val::Px(left) = &mut drag_image_style.left {
        *left += event.delta.x;
    }

    if let Val::Px(top) = &mut drag_image_style.top {
        *top += event.delta.y;
    }

    let window = window_query.single();
    item_drag_events.send(ItemDragEvent::new(
        window,
        drag_image_item_type.item_type.unwrap(),
    ));
}

fn end_drag_inventory_item(
    event: Listener<Pointer<DragEnd>>,
    mut item_drop_events: EventWriter<ItemDropEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut drag_image_query: Query<(&mut Visibility, &mut InventoryDragImage)>,
) {
    if !check_drag_end_event(
        event.listener(),
        event.target,
        event.button,
        PointerButton::Primary,
    ) {
        return;
    }

    let (mut drag_image_visibility, mut drag_image_item_type) = drag_image_query.single_mut();
    *drag_image_visibility = Visibility::Hidden;
    let item_type = drag_image_item_type.item_type.take();

    let window = window_query.single();
    item_drop_events.send(ItemDropEvent::new(window, item_type.unwrap()));
}

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
        "Inventory",
        false,
        InventoryWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Scroll View"),
                ScrollView::default(),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: bevy::ui::FlexDirection::Column,
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Scroll Content"),
                        ScrollableContent::default(),
                        InventoryContent,
                    ))
                    .with_children(|parent| {
                        // Resources
                        create_column_container(parent)
                            .insert(Name::new("Resources"))
                            .with_children(|parent| {
                                create_label(parent, &ui_assets, "Resources", 24.0, FONT_COLOR);

                                create_row_container(parent)
                                    .insert((
                                        Visibility::Hidden,
                                        Name::new("Iron"),
                                        InventoryResourcesUI(ResourceType::Iron),
                                    ))
                                    .with_children(|parent| {
                                        create_image(parent, ui_assets.missing_image.clone());
                                        create_label(parent, &ui_assets, "Iron:", 14.0, FONT_COLOR);
                                        create_label(parent, &ui_assets, "N/A", 14.0, FONT_COLOR)
                                            .insert(InventoryResourcesAmountUI(
                                                ResourceType::Iron,
                                                0,
                                            ));
                                    });
                            });

                        // Items
                        create_column_container(parent)
                            .insert(Name::new("Items"))
                            .with_children(|parent| {
                                create_label(parent, &ui_assets, "Items", 24.0, FONT_COLOR);

                                create_row_container(parent)
                                    .insert((
                                        Visibility::Hidden,
                                        Name::new("Harvesters"),
                                        InventoryItemUI(ItemType::Harvester),
                                    ))
                                    .with_children(|parent| {
                                        let item_image_id = create_draggable_image(
                                            parent,
                                            ui_assets.missing_image.clone(),
                                            On::<Pointer<DragStart>>::run(
                                                start_drag_inventory_item,
                                            ),
                                            On::<Pointer<Drag>>::run(drag_inventory_item),
                                            On::<Pointer<DragEnd>>::run(end_drag_inventory_item),
                                        )
                                        .insert((
                                            InventoryItemImage {
                                                item_type: ItemType::Harvester,
                                            },
                                            Pickable::IGNORE,
                                        ))
                                        .id();

                                        create_label(
                                            parent,
                                            &ui_assets,
                                            "Harvesters:",
                                            14.0,
                                            FONT_COLOR,
                                        );
                                        create_label(parent, &ui_assets, "N/A", 14.0, FONT_COLOR)
                                            .insert(InventoryItemAmountUI(
                                                ItemType::Harvester,
                                                0,
                                                item_image_id,
                                            ));
                                    });
                            });
                    });
            });
    });

    // while we're here, setup the "item drag" image
    commands.spawn((
        ImageBundle {
            style: Style {
                // TODO: don't assume size here
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            image: UiImage::new(ui_assets.missing_image.clone()),
            visibility: Visibility::Hidden,
            ..default()
        },
        Name::new("Inventory Item Drag Image"),
        Pickable::IGNORE,
        InventoryDragImage::default(),
    ));
}

pub(super) fn show_inventory(mut window_query: Query<&mut Visibility, With<InventoryWindow>>) {
    *window_query.single_mut() = Visibility::Visible;
}

#[allow(clippy::type_complexity)]
pub(super) fn inventory_updated_event_handler(
    mut commands: Commands,
    mut events: EventReader<InventoryUpdatedEvent>,
    inventory: Res<Inventory>,
    mut visibility_set: ParamSet<(
        Query<(&mut Visibility, &InventoryResourcesUI)>,
        Query<(&mut Visibility, &InventoryItemUI)>,
    )>,
    mut text_set: ParamSet<(
        Query<(&mut Text, &mut InventoryResourcesAmountUI)>,
        Query<(&mut Text, &mut InventoryItemAmountUI)>,
    )>,
) {
    if events.is_empty() {
        return;
    }

    for (mut visibility, resources) in visibility_set.p0().iter_mut() {
        if inventory.resources.contains_key(&resources.0) {
            *visibility = Visibility::Inherited;
        }
    }

    for (mut text, mut resources) in text_set.p0().iter_mut() {
        if inventory.resources.contains_key(&resources.0) {
            if let Some(amount) = inventory.resources.get(&resources.0) {
                if *amount != resources.1 {
                    text.sections.get_mut(0).unwrap().value = amount.to_string();
                    resources.1 = *amount;
                }
            }
        }
    }

    for (mut visibility, item) in visibility_set.p1().iter_mut() {
        if inventory.items.contains_key(&item.0) {
            *visibility = Visibility::Inherited;
        }
    }

    for (mut text, mut item) in text_set.p1().iter_mut() {
        if inventory.items.contains_key(&item.0) {
            if let Some(amount) = inventory.items.get(&item.0) {
                if *amount != item.1 {
                    text.sections.get_mut(0).unwrap().value = amount.to_string();
                    item.1 = *amount;

                    let mut item_image = commands.entity(item.2);
                    if item.1 == 0 {
                        item_image.insert(Pickable::IGNORE);
                    } else {
                        item_image.remove::<Pickable>();
                    }
                }
            }
        }
    }

    events.clear();
}
