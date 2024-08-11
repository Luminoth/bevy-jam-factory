use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::components::MainCamera;

const CAMERA_SPEED: f32 = 200.0;

pub fn pan_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut _mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    window_query: Query<&Window>,
) {
    let mut camera = camera_query.single_mut();
    let _window = window_query.single();

    let speed = CAMERA_SPEED * time.delta_seconds();

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

    // TODO: need to constrain this to just the visible area
}
