use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;

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

pub fn create_draggable_image<'a>(
    parent: &'a mut ChildBuilder,
    image: Handle<Image>,
    on_drag: On<Pointer<Drag>>,
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
        on_drag,
    ))
}
