use bevy::prelude::*;
use soukoban_rs::SokobanPlugin;

fn main() {
    App::new().add_plugins(SokobanPlugin).run();
}
