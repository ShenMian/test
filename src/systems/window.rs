use bevy::prelude::*;

pub fn setup(mut window: Query<&mut Window>) {
    let mut window = window.get_single_mut().unwrap();
    window.title = "Sokoban".to_string();
}
