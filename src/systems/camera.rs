use bevy::{input::mouse::MouseMotion, prelude::*};

const CAMERA_SPEED: f32 = 10.0;

pub fn pan_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut _mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &Camera2d)>,
) {
    let (mut camera, _) = camera_query.single_mut();

    // TODO: decouple from frame rate
    let speed = CAMERA_SPEED;

    if keys.pressed(KeyCode::ArrowRight) {
        camera.translation.x += speed;
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        camera.translation.x -= speed;
    }

    if keys.pressed(KeyCode::ArrowUp) {
        camera.translation.y += speed;
    }

    if keys.pressed(KeyCode::ArrowDown) {
        camera.translation.y -= speed;
    }
}
