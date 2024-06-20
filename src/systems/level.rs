use std::fs;

use bevy::{prelude::*, utils::HashMap};
use nalgebra::Vector2;
use soukoban::Tiles;

use crate::events;

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
    pub tile_size: Vec2,
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
    mut reset_camera_scale_events: EventWriter<events::ResetCameraScale>,
    mut reset_camera_translate_events: EventWriter<events::ResetCameraTranslate>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    let level = soukoban::Level::load_nth_from_string(
        &fs::read_to_string("assets/levels/box_world_100.xsb").unwrap(),
        level_id.0,
    )
    .unwrap();
    commands
        .spawn((Level(level.clone()), TransformBundle::default()))
        .with_children(|parent| {
            for y in 0..level.dimensions().y {
                for x in 0..level.dimensions().x {
                    let position = Vector2::new(x, y);
                    if level[position].is_empty() {
                        continue;
                    }
                    for tile in level[position] {
                        let (sprite_index, z_order) = tilesheet.tile_info[&tile];
                        parent.spawn(SpriteSheetBundle {
                            atlas: TextureAtlas {
                                layout: tilesheet.layout_handle.clone(),
                                index: sprite_index,
                            },
                            texture: tilesheet.handle.clone(),
                            transform: Transform::from_xyz(
                                x as f32 * tilesheet.tile_size.x,
                                -y as f32 * tilesheet.tile_size.y, // Quadrant 4
                                z_order,
                            ),
                            ..default()
                        });
                    }
                }
            }
        });

    reset_camera_scale_events.send_default();
    reset_camera_translate_events.send_default();

    info!("Level #{}", level_id.0)
}
