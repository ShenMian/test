use bevy::{prelude::*, window::WindowResized};

use crate::systems::level::{Level, Tilesheet};

#[derive(Event, Default)]
pub struct ResetCameraScale;

#[derive(Event, Default)]
pub struct ResetCameraTranslate;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                handle_window_resized_event.run_if(on_event::<WindowResized>()),
                handle_reset_camera_scale_event.run_if(on_event::<ResetCameraScale>()),
                handle_reset_camera_translate_event.run_if(on_event::<ResetCameraTranslate>()),
            )
                .chain(),
        );
        app.add_event::<ResetCameraScale>()
            .add_event::<ResetCameraTranslate>();
    }
}

/// Sets up the main 2D camera.
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn handle_reset_camera_scale_event(
    mut events: EventReader<ResetCameraScale>,
    mut projection: Query<&mut OrthographicProjection>,
    tilesheet: Res<Tilesheet>,
    level: Query<&Level>,
    window: Query<&Window>,
) {
    events.clear();

    let Ok(level) = level.get_single() else {
        return;
    };
    let map = level.map();
    let size = tilesheet.tile_size.x as f32 * map.dimensions().map(|x| x as f32);

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
    mut events: EventReader<ResetCameraTranslate>,
    tilesheet: Res<Tilesheet>,
    level: Query<&Level>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    events.clear();

    let Ok(level) = level.get_single() else {
        return;
    };
    let map = level.map();
    let size = tilesheet.tile_size.x as f32 * map.dimensions().map(|x| x as f32);

    let mut camera = camera.single_mut();
    camera.translation.x = (size.x - tilesheet.tile_size.x as f32) / 2.0;
    camera.translation.y = -((size.y - tilesheet.tile_size.y as f32) / 2.0);
}

pub fn handle_window_resized_event(
    mut events: EventReader<WindowResized>,
    mut reset_camera_scale_events: EventWriter<ResetCameraScale>,
) {
    events.clear();
    reset_camera_scale_events.send_default();
}
