use std::path::PathBuf;

use bevy::{prelude::*, window::WindowResized};

mod plugins;
mod systems;

use plugins::*;

use cursor::IconPath;
use systems::level::{LevelId, Tilesheet};

pub struct SokobanPlugin;

impl Plugin for SokobanPlugin {
    fn build(&self, app: &mut App) {
        // #[cfg(debug_assertions)]
        app.add_plugins(performance_matrix::PerformanceMatrixPlugin);

        app.add_plugins(cursor::CursorPlugin {
            config: IconPath {
                default_icon_path: PathBuf::from("textures/cursor/Outline/Default/pointer_b.png"),
                pointer_icon_path: PathBuf::from("textures/cursor/Outline/Default/hand_point.png"),
            },
        });

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
            (
                systems::level::respawn.run_if(resource_changed_or_removed::<LevelId>()),
                systems::camera::handle_window_resized_event.run_if(on_event::<WindowResized>()),
                systems::camera::handle_reset_camera_scale_event
                    .run_if(on_event::<systems::camera::ResetCameraScale>()),
                systems::camera::handle_reset_camera_translate_event
                    .run_if(on_event::<systems::camera::ResetCameraTranslate>()),
            )
                .chain(),
        );

        app.init_resource::<Tilesheet>().init_resource::<LevelId>();

        app.add_event::<systems::camera::ResetCameraScale>()
            .add_event::<systems::camera::ResetCameraTranslate>();
    }
}
