use std::collections::HashMap;

use bevy::prelude::*;
use nalgebra::Vector2;
use soukoban::Tiles;

use crate::systems::camera::*;

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
    pub tile_size: UVec2,
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
            tile_size: UVec2::new(128, 128),
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
    tilesheet.tile_size = UVec2::new(128, 128);
    let layout =
        TextureAtlasLayout::from_grid(tilesheet.tile_size, 6, 3, Some(UVec2::new(1, 1)), None);
    tilesheet.layout_handle = texture_atlas_layouts.add(layout);
}

pub fn respawn(
    mut commands: Commands,
    level_id: Res<LevelId>,
    tilesheet: Res<Tilesheet>,
    query: Query<Entity, With<Level>>,
    mut reset_camera_scale_events: EventWriter<ResetCameraScale>,
    mut reset_camera_translate_events: EventWriter<ResetCameraTranslate>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    let level = soukoban::Level::load_nth_from_string(
        std::include_str!("../../assets/levels/box_world_100.xsb"),
        level_id.0,
    )
    .unwrap();
    let map = level.map();

    commands
        .spawn((Level(level.clone()), TransformBundle::default()))
        .with_children(|parent| {
            for y in 0..map.dimensions().y {
                for x in 0..map.dimensions().x {
                    let position = Vector2::new(x, y);
                    if map[position].is_empty() {
                        continue;
                    }
                    for tile in map[position] {
                        let (sprite_index, z_order) = tilesheet.tile_info[&tile];
                        parent.spawn((
                            SpriteBundle {
                                texture: tilesheet.handle.clone(),
                                transform: Transform::from_xyz(
                                    x as f32 * tilesheet.tile_size.x as f32,
                                    -y as f32 * tilesheet.tile_size.y as f32, // Quadrant 4
                                    z_order,
                                ),
                                ..default()
                            },
                            TextureAtlas {
                                layout: tilesheet.layout_handle.clone(),
                                index: sprite_index,
                            },
                        ));
                    }
                }
            }
        });

    reset_camera_scale_events.send_default();
    reset_camera_translate_events.send_default();

    info!("Level #{}", level_id.0)
}
