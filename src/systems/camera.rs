use bevy::prelude::*;

/// Sets up the main 2D camera.
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
