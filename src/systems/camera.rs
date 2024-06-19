use bevy::prelude::*;

use crate::events;

/// Sets up the main 2D camera.
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn handle_reset_camera_scale_event(mut event_reader: EventReader<events::ResetCameraScale>) {
    event_reader.clear();
    todo!()
}
