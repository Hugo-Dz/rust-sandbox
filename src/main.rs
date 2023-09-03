mod utils;

use crate::utils::remap::remap_cursor_position;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PresentMode;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use rand::Rng;

const GAME_RESOLUTION_X: usize = 80;
const GAME_RESOLUTION_Y: usize = 60;
const WINDOW_SIZE: [f32; 2] = [640.0, 480.0];

#[derive(Resource)]
struct TickCounter {
    count: u32,
    tick_rate: u32,
}

#[derive(Resource)]
struct Grid {
    data: [[Option<GrainType>; GAME_RESOLUTION_X]; GAME_RESOLUTION_Y],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            data: [[None; GAME_RESOLUTION_X]; GAME_RESOLUTION_Y],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<GrainType> {
        if x < GAME_RESOLUTION_X && y < GAME_RESOLUTION_Y {
            self.data[y][x]
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: GrainType) {
        if x < 80 && y < 60 {
            self.data[y][x] = Some(value);
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Sandy".into(),
                        resolution: (WINDOW_SIZE[0], WINDOW_SIZE[1]).into(),
                        resizable: false,
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            PixelCameraPlugin,
        ))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(TickCounter { count: 0, tick_rate: 1 })
        .insert_resource(Grid::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (add_grain, drop_grain, update_grid_data))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(
        GAME_RESOLUTION_X as i32,
        GAME_RESOLUTION_Y as i32,
        true,
    ));
}

#[derive(Component)]
struct Grain;

#[derive(Component)]
struct Name;

#[derive(Component, Clone, Copy, Debug)]
enum GrainType {
    Sand,
    _Rock,
    _Water,
}

#[derive(Component)]
struct GridPosition {
    current_x: i32,
    current_y: i32,
    prev_x: Option<i32>,
    prev_y: Option<i32>,
}

fn add_grain(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    query: Query<&Window>,
) {
    let sand_textures: [Handle<Image>; 3] = [
        asset_server.load("sand1.png"),
        asset_server.load("sand2.png"),
        asset_server.load("sand3.png"),
    ];
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..sand_textures.len());
    let texture = sand_textures[random_index].clone();

    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        texture,
        ..default()
    };

    if let Some(position) = query.single().cursor_position() {
        if input.pressed(MouseButton::Left) {
            let remaped_cursor_pos =
                remap_cursor_position(position, WINDOW_SIZE, [GAME_RESOLUTION_X, GAME_RESOLUTION_Y]);
            //println!("{}", remaped_cursor_pos);
            // Add a row (entity) with this set of components
            commands
                .spawn((
                    Grain,
                    sprite_bundle,
                    GrainType::Sand,
                    GridPosition {
                        current_x: remaped_cursor_pos.x.round() as i32,
                        current_y: remaped_cursor_pos.y.round() as i32,
                        prev_x: None,
                        prev_y: None,
                    },
                ))
                .insert(Transform {
                    translation: Vec3::new(
                        remaped_cursor_pos.x.round() - (GAME_RESOLUTION_X as f32 / 2.0),
                        -(remaped_cursor_pos.y.round() - (GAME_RESOLUTION_Y as f32 / 2.0)),
                        0.0,
                    ),
                    ..default()
                });
        }
    }
}

fn drop_grain(
    mut query: Query<(&mut Transform, &mut GridPosition)>,
    mut tick_counter: ResMut<TickCounter>,
    grid_data: Res<Grid>,
) {
    tick_counter.count += 1;

    if tick_counter.count >= tick_counter.tick_rate {
        tick_counter.count = 0;

        for (mut transform, mut grid_position) in query.iter_mut() {
            if transform.translation.y >= -(GAME_RESOLUTION_Y as f32 / 2.0 - 2.0) {
                // Check if there is no grain below
                if grid_data
                    .get(grid_position.current_x as usize, (grid_position.current_y + 1) as usize)
                    .is_none()
                {
                    transform.translation.y -= 1.0;

                    // Update grid_position
                    grid_position.prev_x = Some(grid_position.current_x);
                    grid_position.prev_y = Some(grid_position.current_y);

                    grid_position.current_y += 1;
                }
            }
        }
    }
}

fn update_grid_data(mut query: Query<(&GrainType, &mut GridPosition)>, mut grid_data: ResMut<Grid>) {
    for (grain_type, mut pos) in query.iter_mut() {
        // Clear the previous position from the grid
        if let (Some(prev_x), Some(prev_y)) = (pos.prev_x, pos.prev_y) {
            if prev_x >= 0 && prev_y >= 0 {
                grid_data.data[prev_y as usize][prev_x as usize] = None;
            }
        }

        // Update the grid with the new position
        grid_data.set(pos.current_x as usize, pos.current_y as usize, *grain_type);

        // Update prev for next frame
        pos.prev_x = Some(pos.current_x);
        pos.prev_y = Some(pos.current_y);
    }
}
