use bevy::prelude::*;
use bevy_tweening::Tween;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u64)]
pub enum TweenId {
    HideDragImage = 1,
}

pub fn simple_tween_ui_object(
    duration_ms: u64,
    start: (Val, Val),
    end: (Val, Val),
    completed_event: TweenId,
) -> Tween<Style> {
    Tween::new(
        bevy_tweening::EaseFunction::QuadraticOut,
        std::time::Duration::from_millis(duration_ms),
        bevy_tweening::lens::UiPositionLens {
            start: UiRect {
                left: start.0,
                top: start.1,
                right: Val::Auto,
                bottom: Val::Auto,
            },
            end: UiRect {
                left: end.0,
                top: end.1,
                right: Val::Auto,
                bottom: Val::Auto,
            },
        },
    )
    // TODO: this really sucks lol
    .with_completed_event(completed_event as u64)
}
