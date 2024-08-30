use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;

#[allow(dead_code)]
pub fn create_image<'a>(parent: &'a mut ChildBuilder, image: Handle<Image>) -> EntityCommands<'a> {
    parent.spawn((
        ImageBundle {
            style: Style {
                // TODO: don't assume size here
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            image: UiImage::new(image),
            ..default()
        },
        Name::new("Image"),
        Pickable::IGNORE,
    ))
}

pub fn create_image_from_slice<'a>(
    parent: &'a mut ChildBuilder,
    image: Handle<Image>,
    atlas: Handle<TextureAtlasLayout>,
    index: usize,
) -> EntityCommands<'a> {
    parent.spawn((
        ImageBundle {
            style: Style {
                // TODO: don't assume size here
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            image: UiImage::new(image),
            ..default()
        },
        TextureAtlas {
            layout: atlas,
            index,
        },
        Name::new("Image"),
        Pickable::IGNORE,
    ))
}

#[allow(dead_code)]
pub fn create_draggable_image<'a>(
    parent: &'a mut ChildBuilder,
    image: Handle<Image>,
    on_drag_start: On<Pointer<DragStart>>,
    on_drag: On<Pointer<Drag>>,
    on_drag_end: On<Pointer<DragEnd>>,
) -> EntityCommands<'a> {
    parent.spawn((
        ImageBundle {
            style: Style {
                // TODO: don't assume size here
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            image: UiImage::new(image),
            ..default()
        },
        Name::new("Draggable Image"),
        on_drag_start,
        on_drag,
        on_drag_end,
    ))
}

pub fn create_draggable_image_from_slice<'a>(
    parent: &'a mut ChildBuilder,
    image: Handle<Image>,
    atlas: Handle<TextureAtlasLayout>,
    index: usize,
    on_drag_start: On<Pointer<DragStart>>,
    on_drag: On<Pointer<Drag>>,
    on_drag_end: On<Pointer<DragEnd>>,
) -> EntityCommands<'a> {
    parent.spawn((
        ImageBundle {
            style: Style {
                // TODO: don't assume size here
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            image: UiImage::new(image),
            ..default()
        },
        TextureAtlas {
            layout: atlas,
            index,
        },
        Name::new("Draggable Image"),
        on_drag_start,
        on_drag,
        on_drag_end,
    ))
}
