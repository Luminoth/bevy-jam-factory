use bevy::{ecs::system::EntityCommands, prelude::*};

#[allow(dead_code)]
pub fn create_image<'a>(parent: &'a mut ChildBuilder, image: Handle<Image>) -> EntityCommands<'a> {
    parent.spawn((
        Node {
            // TODO: don't assume size here
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            ..default()
        },
        ImageNode::new(image),
        Name::new("Image"),
    ))
}

pub fn create_image_from_slice<'a>(
    parent: &'a mut ChildBuilder,
    image: Handle<Image>,
    atlas: Handle<TextureAtlasLayout>,
    index: usize,
) -> EntityCommands<'a> {
    parent.spawn((
        Node {
            // TODO: don't assume size here
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            ..default()
        },
        ImageNode::from_atlas_image(
            image,
            TextureAtlas {
                layout: atlas,
                index,
            },
        ),
        Name::new("Image"),
    ))
}
