use std::path::PathBuf;

use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct IconPath {
    pub default_icon_path: PathBuf,
    pub pointer_icon_path: PathBuf,
}

pub struct CursorPlugin {
    pub config: IconPath,
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone());
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_cursor, switch_cursor_icon));
    }
}

#[derive(Default)]
enum CursorIcon {
    #[default]
    Default,
    // Pointer,
}

#[derive(Component, Deref, DerefMut, Default)]
struct Cursor(CursorIcon);

fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;

    commands.spawn((
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Global(1),
            ..default()
        },
        Cursor::default(),
    ));
}

fn move_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<Cursor>>) {
    let window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut style = cursor.single_mut();
        let offset = Vec2::new(-10.0, -6.0);
        style.left = Val::Px(position.x + offset.x);
        style.top = Val::Px(position.y + offset.y);
    }
}

fn switch_cursor_icon(
    mut query: Query<(&Cursor, &mut UiImage), Changed<Cursor>>,
    asset_server: Res<AssetServer>,
    icon_path: Res<IconPath>,
) {
    for (cursor, mut image) in query.iter_mut() {
        image.texture = match cursor.0 {
            CursorIcon::Default => asset_server.load(icon_path.default_icon_path.clone()),
            CursorIcon::Pointer => asset_server.load(icon_path.pointer_icon_path.clone()),
        };
    }
}
