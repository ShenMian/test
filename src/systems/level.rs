use std::fs;

use bevy::{prelude::*, utils::HashMap};
use soukoban::Tiles;

#[derive(Component, Deref, DerefMut)]
pub struct Level(soukoban::Level);

#[derive(Resource, Deref, DerefMut)]
pub struct LevelId(pub usize);

impl Default for LevelId {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Resource)]
pub struct Tilesheet {
    tile_size: Vec2,
    tile_info: HashMap<Tiles, (usize, f32)>,
    handle: Handle<Image>,
    layout_handle: Handle<TextureAtlasLayout>,
}

impl Default for Tilesheet {
    fn default() -> Self {
        let tile_info = HashMap::from([
            (Tiles::Floor, (0, 0.0)),
            (Tiles::Wall, (3, 1.0)),
            (Tiles::Box, (1, 2.0)),
            (Tiles::Goal, (2, 3.0)),
            (Tiles::Player, (0, 4.0)),
        ]);
        Self {
            tile_size: Vec2::new(128.0, 128.0),
            tile_info,
            handle: Handle::default(),
            layout_handle: Handle::default(),
        }
    }
}

pub fn load_assets(
    mut tilesheet: ResMut<Tilesheet>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    tilesheet.handle = asset_server.load("textures/tilesheet.png");
    tilesheet.tile_size = Vec2::new(128.0, 128.0);
    let layout =
        TextureAtlasLayout::from_grid(tilesheet.tile_size, 6, 3, Some(Vec2::new(1.0, 1.0)), None);
    tilesheet.layout_handle = texture_atlas_layouts.add(layout);
}

pub fn respawn(
    mut commands: Commands,
    level_id: Res<LevelId>,
    tilesheet: Res<Tilesheet>,
    query: Query<Entity, With<Level>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    let level = soukoban::Level::load_nth_from_string(
        &fs::read_to_string("assets/levels/box_world_100.xsb").unwrap(),
        level_id.0,
    )
    .unwrap();
    commands.spawn(Level(level)).with_children(|parent| {
        let z = 0.0;
        let index = 0;
        parent.spawn((SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: tilesheet.layout_handle.clone(),
                index,
            },
            texture: tilesheet.handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, z),
            ..default()
        },));
    });

    println!("Level #{}", level_id.0)
}
