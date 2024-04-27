use bevy::prelude::*;

mod systems;

#[cfg(debug_assertions)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

pub struct SokobanPlugin;

impl Plugin for SokobanPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);

        #[cfg(debug_assertions)]
        app.add_plugins(FrameTimeDiagnosticsPlugin);

        app.add_systems(Startup, systems::window::setup_window);
    }
}
