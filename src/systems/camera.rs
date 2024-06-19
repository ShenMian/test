use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::events;
use crate::systems::level::{Level, Tilesheet};

/// Sets up the main 2D camera.
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn handle_reset_camera_scale_event(
    mut events: EventReader<events::ResetCameraScale>,
    mut projection: Query<&mut OrthographicProjection>,
    tilesheet: Res<Tilesheet>,
    level: Query<&Level>,
    window: Query<&Window>,
) {
    events.clear();

    let level = level.single();
    let size = tilesheet.tile_size.x * level.dimensions().map(|x| x as f32);

    let window = window.single();
    let width_scale = size.x / window.resolution.width();
    let height_scale = size.y / window.resolution.height();
    let scale = if width_scale > height_scale {
        width_scale
    } else {
        height_scale
    };
    projection.single_mut().scale = scale / 0.9;
}

pub fn handle_reset_camera_translate_event(
    mut events: EventReader<events::ResetCameraTranslate>,
    tilesheet: Res<Tilesheet>,
    level: Query<&Level>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    events.clear();

    let level = level.single();
    let size = tilesheet.tile_size.x * level.dimensions().map(|x| x as f32);

    let mut camera = camera.single_mut();
    camera.translation.x = (size.x - tilesheet.tile_size.x) / 2.0;
    camera.translation.y = -(size.y - tilesheet.tile_size.y) / 2.0;
}

pub fn handle_window_resized_event(
    mut events: EventReader<WindowResized>,
    mut reset_camera_scale_events: EventWriter<events::ResetCameraScale>,
) {
    events.clear();
    reset_camera_scale_events.send_default();
}
