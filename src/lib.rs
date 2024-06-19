use bevy::prelude::*;

mod performance_matrix;
mod systems;

pub struct SokobanPlugin;

impl Plugin for SokobanPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(DefaultPlugins);

        // #[cfg(debug_assertions)]
        app.add_plugins(performance_matrix::PerformanceMatrixPlugin);

        app.add_systems(
            Startup,
            (
                systems::window::setup,
                systems::camera::setup,
                systems::level::load_assets,
            ),
        );
        app.add_systems(
            Update,
            systems::level::respawn.run_if(resource_changed_or_removed::<LevelId>()),
        );

        app.init_resource::<systems::level::Tilesheet>();
    }
}
