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
const TICK_RATE: u32 = 1;
const BLOOD_SAFE_TIME: u32 = 200; // Time to stop blood interaction to avoid infinite back and forth movement

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
        if x < GAME_RESOLUTION_X && y < GAME_RESOLUTION_Y {
            self.data[y][x] = Some(value);
        }
    }
}

#[derive(Component)]
struct Grain;

#[derive(Component)]
struct Lifetime(u32);

#[derive(Component, Clone, Copy, Debug)]
enum GrainType {
    Bone,
    Blood,
}

#[derive(Component)]
struct GridPosition {
    current_x: i32,
    current_y: i32,
    prev_x: Option<i32>,
    prev_y: Option<i32>,
}

#[derive(Component, PartialEq)]
enum Direction {
    Right,
    Left,
}

// Main function
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Hell".into(),
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
        .insert_resource(TickCounter { count: 0, tick_rate: TICK_RATE })
        .insert_resource(Grid::new())
        .add_systems(Startup, setup)
        .add_systems(Update, add_grain)
        .add_systems(Update, update_grain_system)
        .add_systems(Update, update_grid_data)
        .add_systems(Update, shade_blood)
        .run();
}

// Setup and spawn the camera
fn setup(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(
        GAME_RESOLUTION_X as i32,
        GAME_RESOLUTION_Y as i32,
        true,
    ));
}

// Listen for mouse clicks to spawn grains
fn add_grain(
    mut commands: Commands,
    query: Query<&Window>,
    asset_server: Res<AssetServer>,
    input: Res<Input<MouseButton>>
) {

    if let Some(position) = query.single().cursor_position() {

        let remaped_cursor_pos = remap_cursor_position(position, WINDOW_SIZE, [GAME_RESOLUTION_X, GAME_RESOLUTION_Y]);
        let mut rng = rand::thread_rng();

        if input.pressed(MouseButton::Left) {

            // Create a bone sprite texture
            let bone_textures: [&str; 3] = [
                "bone_1.png",
                "bone_2.png",
                "bone_3.png"
            ];
            let random_index = rng.gen_range(0..bone_textures.len());
        
            let bone_sprite_bundle = SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture: asset_server.load(bone_textures[random_index]),
                ..default()
            };

            // Add a data row (entity) with it's set of components
            commands
                .spawn((
                    Grain,
                    bone_sprite_bundle,
                    GrainType::Bone,
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

        if input.pressed(MouseButton::Right) {

            // Creat a blood sprite texture
            let blood_sprite_bundle = SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture: asset_server.load("blood_2.png"),
                ..default()
            };

            // Pick a random direction to slide toward
            let random_number = rng.gen::<f32>();

            let random_direction = if random_number < 0.5 {
                Direction::Left
            } else {
                Direction::Right
            };

            // Add a data row (entity) with it's set of components
            commands
                .spawn((
                    Grain,
                    Lifetime(0),
                    blood_sprite_bundle,
                    GrainType::Blood,
                    random_direction,
                    GridPosition {
                        current_x: remaped_cursor_pos.x.round() as i32,
                        current_y: remaped_cursor_pos.y.round() as i32,
                        prev_x: None,
                        prev_y: None,
                    }
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

fn handle_bone_grain(grid_position: &GridPosition, grid_data: &Grid) {
    // Ensure coordinates are in the grid
    if is_within_boundaries(grid_position, GAME_RESOLUTION_X as i32, GAME_RESOLUTION_Y as i32) {

        let maybe_grain_above = grid_data.get(grid_position.current_x as usize, (grid_position.current_y - 1) as usize);

        if let Some(grain_above) = maybe_grain_above {
            match grain_above {
                GrainType::Blood => {
                    // Not used but some ideas:
                    // Tint the bone to red-ish color
                    // Break the bone after X lifetime (think to add the lifetime component to the bone entity)
                },
                _ => {}
            }
        }
    }
}

fn is_within_boundaries(grid_position: &GridPosition, res_x: i32, res_y: i32) -> bool {
    grid_position.current_x >= 1 && grid_position.current_x < res_x - 1 && grid_position.current_y >= 0 && grid_position.current_y < res_y - 1
}

fn move_grain(transform: &mut Transform, grid_position: &mut GridPosition, x: i32, y: i32) {
    transform.translation.x += x as f32;
    transform.translation.y -= y as f32;
    grid_position.prev_x = Some(grid_position.current_x);
    grid_position.prev_y = Some(grid_position.current_y);
    grid_position.current_x += x;
    grid_position.current_y += y;
}

fn handle_blood_grain(
    transform: &mut Transform,
    grid_position: &mut GridPosition,
    grid_data: &Grid,
    direction: &mut Direction,
    lifetime: u32
) {
    // Ensure coordinates are in the grid
    if is_within_boundaries(grid_position, GAME_RESOLUTION_X as i32, GAME_RESOLUTION_Y as i32) {

        let maybe_grain_below = grid_data.get(grid_position.current_x as usize, (grid_position.current_y + 1) as usize);
        let maybe_grain_right = grid_data.get((grid_position.current_x + 1) as usize, grid_position.current_y as usize);
        let maybe_grain_left = grid_data.get((grid_position.current_x - 1) as usize, grid_position.current_y as usize);

        match maybe_grain_below {
            Some(_) => {

                // Do nothing (to avoid grain behavior in the air just after spawning)
                if lifetime < 5 {
                    return;
                }

                // There is a grain below, try moving
                match (maybe_grain_left, maybe_grain_right) {
                    (Some(_), None) => match direction {
                        Direction::Left => {
                            if lifetime < BLOOD_SAFE_TIME {
                                *direction = Direction::Right;
                            }
                        }
                        Direction::Right => move_grain(transform, grid_position, 1, 0)
                    },
                    (None, Some(_)) => match direction {
                        Direction::Left => move_grain(transform, grid_position, -1, 0),
                        Direction::Right => {
                            if lifetime < BLOOD_SAFE_TIME {
                                *direction = Direction::Left;
                            }
                        }
                    },
                    (None, None) => {
                        match *direction {
                            Direction::Right => move_grain(transform, grid_position, 1, 0),
                            Direction::Left => move_grain(transform, grid_position, -1, 0)
                        }
                    }
                    (Some(_), Some(_)) => { /* Do nothing, stay in place */ }
                }
            }
            None => {
                // No grain below, just fall
                move_grain(transform, grid_position, 0, 1)
            }
        }
    }
}

fn update_grain_system(
    grid_data: Res<Grid>,
    mut query: Query<(
        &mut Transform,
        &mut GridPosition,
        &GrainType,
        &mut Lifetime,
        Option<&mut Direction>,
    )>,
    mut tick_counter: ResMut<TickCounter>,
) {
    tick_counter.count += 1;
    if tick_counter.count >= tick_counter.tick_rate {
        tick_counter.count = 0;
        for (mut transform, mut grid_position, grain_type, mut lifetime, direction) in query.iter_mut() {
            lifetime.0 += 1;
            match grain_type {
                GrainType::Bone => handle_bone_grain(&grid_position, &grid_data),
                GrainType::Blood => {
                    if let Some(mut dir) = direction {
                        handle_blood_grain(&mut transform, &mut grid_position, &grid_data, &mut dir, lifetime.0);
                    }
                }
            }
        }
    }
}

fn shade_blood(
    mut query: Query<(&GrainType, &mut GridPosition, &mut Handle<Image>)>,
    grid_data: ResMut<Grid>,
    asset_server: Res<AssetServer>,
) {
    for (grain_type, grid_position, mut texture_handle) in query.iter_mut() {

        let maybe_grain_above = grid_data.get(grid_position.current_x as usize, (grid_position.current_y - 1) as usize);

        match grain_type {
            GrainType::Blood => match maybe_grain_above {
                None => {
                    *texture_handle = asset_server.load("blood_1.png");
                }
                _ => {
                    *texture_handle = asset_server.load("blood_2.png");
                }
            },
            _ => {}
        }
    }
}

fn update_grid_data(mut query: Query<(&GrainType, &mut GridPosition)>, mut grid_data: ResMut<Grid>) {
    for (grain_type, mut grid_position) in query.iter_mut() {
        // Clear the previous position from the grid
        if let (Some(prev_x), Some(prev_y)) = (grid_position.prev_x, grid_position.prev_y) {
            if prev_x >= 0 && prev_y >= 0 && prev_x < GAME_RESOLUTION_X as i32 && prev_y < GAME_RESOLUTION_Y as i32 {
                grid_data.data[prev_y as usize][prev_x as usize] = None;
            }
        }

        // Boundary checks for current positions
        if grid_position.current_x >= 0
            && grid_position.current_x < GAME_RESOLUTION_X as i32
            && grid_position.current_y >= 0
            && grid_position.current_y < GAME_RESOLUTION_Y as i32
        {
            // Update the grid with the new position
            grid_data.set(grid_position.current_x as usize, grid_position.current_y as usize, *grain_type);
        }

        // Update prev for next frame
        grid_position.prev_x = Some(grid_position.current_x);
        grid_position.prev_y = Some(grid_position.current_y);
    }
}
