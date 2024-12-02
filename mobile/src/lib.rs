use bevy::{prelude::*, window::WindowMode};
use soukoban_rs::SokobanPlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            SokobanPlugin,
        ))
        .run();
}
