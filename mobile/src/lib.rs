use bevy::prelude::*;
use soukoban_rs::SokobanPlugin;

#[bevy_main]
fn main() {
    App::new().add_plugins(SokobanPlugin).run()
}
