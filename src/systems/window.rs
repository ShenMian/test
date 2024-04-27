use bevy::prelude::*;

#[cfg(target_os = "android")]
use bevy::window::WindowMode;

pub fn setup_window(mut window: Query<&mut Window>) {
    let mut window = window.get_single_mut().unwrap();
    window.title = "Sokoban".to_string();

    #[cfg(target_os = "android")]
    {
        window.mode = WindowMode::BorderlessFullscreen;
        window.resizable = false;
    }
}
