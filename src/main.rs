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
        if x < GAME_RESOLUTION_X && y < GAME_RESOLUTION_Y {
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
        .add_systems(Update, add_grain)
        .add_systems(Update, update_grain_system)
        .add_systems(Update, update_grid_data)
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
struct Lifetime(u32);

#[derive(Component, Clone, Copy, Debug)]
enum GrainType {
    Sand,
    _Rock,
    Water,
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
    let sand_texture = sand_textures[random_index].clone();

    let sand_sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        texture: sand_texture,
        ..default()
    };

    let water_textures: [Handle<Image>; 3] = [
        asset_server.load("water1.png"),
        asset_server.load("water2.png"),
        asset_server.load("water3.png"),
    ];
    let water_texture = water_textures[random_index].clone();

    let water_sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        texture: water_texture,
        ..default()
    };

    if let Some(position) = query.single().cursor_position() {
        let remaped_cursor_pos = remap_cursor_position(position, WINDOW_SIZE, [GAME_RESOLUTION_X, GAME_RESOLUTION_Y]);

        if input.pressed(MouseButton::Left) {
            // Add a row (entity) with this set of components
            commands
                .spawn((
                    Grain,
                    Lifetime(0),
                    sand_sprite_bundle,
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

        if input.pressed(MouseButton::Right) {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen::<f64>();

            let random_direction = if random_number < 0.5 {
                Direction::Left
            } else {
                Direction::Right
            };

            commands
                .spawn((
                    Grain,
                    Lifetime(0),
                    water_sprite_bundle,
                    GrainType::Water,
                    random_direction,
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

fn handle_sand_grain(transform: &mut Transform, grid_position: &mut GridPosition, grid_data: &Grid, lifetime: u32) {
    // Ensure coordinates are in the grid
    if grid_position.current_x >= 1
        && grid_position.current_x < GAME_RESOLUTION_X as i32 - 1
        && grid_position.current_y >= 0
        && grid_position.current_y < GAME_RESOLUTION_Y as i32 - 1
    {
        let maybe_grain_below = grid_data.get(grid_position.current_x as usize, (grid_position.current_y + 1) as usize);
        let maybe_grain_right = grid_data.get((grid_position.current_x + 1) as usize, grid_position.current_y as usize);
        let maybe_grain_left = grid_data.get((grid_position.current_x - 1) as usize, grid_position.current_y as usize);

        let mut rng = rand::thread_rng();
        let random_number = rng.gen::<f64>();

        match maybe_grain_below {
            Some(_) => {
                if lifetime > 500 {
                    return;
                }
                // There is a grain below, try moving
                match (maybe_grain_left, maybe_grain_right) {
                    (Some(_), None) => {
                        if random_number < 0.01 {
                            transform.translation.x += 1.0;
                            grid_position.prev_x = Some(grid_position.current_x);
                            grid_position.prev_y = Some(grid_position.current_y);
                            grid_position.current_x += 1;
                        }
                    }
                    (None, Some(_)) => {
                        if random_number < 0.01 {
                            transform.translation.x -= 1.0;
                            grid_position.prev_x = Some(grid_position.current_x);
                            grid_position.prev_y = Some(grid_position.current_y);
                            grid_position.current_x -= 1;
                        }
                    }
                    (None, None) => {
                        if random_number < 0.80 {
                            if random_number < 0.40 {
                                transform.translation.x += 1.0;
                                grid_position.prev_x = Some(grid_position.current_x);
                                grid_position.prev_y = Some(grid_position.current_y);
                                grid_position.current_x += 1;
                            } else {
                                transform.translation.x -= 1.0;
                                grid_position.prev_x = Some(grid_position.current_x);
                                grid_position.prev_y = Some(grid_position.current_y);
                                grid_position.current_x -= 1;
                            }
                        }
                    }
                    (Some(_), Some(_)) => {}
                }
            }
            None => {
                // No grain below, just fall
                transform.translation.y -= 1.0;
                grid_position.prev_x = Some(grid_position.current_x);
                grid_position.prev_y = Some(grid_position.current_y);
                grid_position.current_y += 1;
            }
        }
    }
}

fn handle_water_grain(
    transform: &mut Transform,
    grid_position: &mut GridPosition,
    grid_data: &Grid,
    direction: &mut Direction
) {
    // Ensure coordinates are in the grid
    if grid_position.current_x >= 1
        && grid_position.current_x < GAME_RESOLUTION_X as i32 - 1
        && grid_position.current_y >= 0
        && grid_position.current_y < GAME_RESOLUTION_Y as i32 - 1
    {
        let maybe_grain_below = grid_data.get(grid_position.current_x as usize, (grid_position.current_y + 1) as usize);
        let maybe_grain_right = grid_data.get((grid_position.current_x + 1) as usize, grid_position.current_y as usize);
        let maybe_grain_left = grid_data.get((grid_position.current_x - 1) as usize, grid_position.current_y as usize);

        match maybe_grain_below {
            Some(_) => {
                // There is a grain below, try moving
                match (maybe_grain_left, maybe_grain_right) {
                    (Some(_), None) => match direction {
                        Direction::Left => {},
                        Direction::Right => {
                            transform.translation.x += 1.0;
                            grid_position.prev_x = Some(grid_position.current_x);
                            grid_position.prev_y = Some(grid_position.current_y);
                            grid_position.current_x += 1;
                        }
                    },
                    (None, Some(_)) => match direction {
                        Direction::Left => {
                            transform.translation.x -= 1.0;
                            grid_position.prev_x = Some(grid_position.current_x);
                            grid_position.prev_y = Some(grid_position.current_y);
                            grid_position.current_x -= 1;
                        },
                        Direction::Right => {}
                    },
                    (None, None) => {
                        if *direction == Direction::Right {
                            transform.translation.x += 1.0;
                            grid_position.prev_x = Some(grid_position.current_x);
                            grid_position.prev_y = Some(grid_position.current_y);
                            grid_position.current_x += 1;
                        }
                        if *direction == Direction::Left {
                            transform.translation.x -= 1.0;
                            grid_position.prev_x = Some(grid_position.current_x);
                            grid_position.prev_y = Some(grid_position.current_y);
                            grid_position.current_x -= 1;
                        }
                    }
                    (Some(_), Some(_)) => { /* Do nothing, stay in place */ }
                }
            }
            None => {
                // No grain below, just fall
                transform.translation.y -= 1.0;
                grid_position.prev_x = Some(grid_position.current_x);
                grid_position.prev_y = Some(grid_position.current_y);
                grid_position.current_y += 1;
            }
        }
    }
}

// TODO - Maybe do a system per grain type? To avoid Option<LastDirection)> that concern only water grain
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
                GrainType::Sand => handle_sand_grain(&mut transform, &mut grid_position, &grid_data, lifetime.0),
                GrainType::_Rock => { /* handle rock logic */ }
                GrainType::Water => {
                    if let Some(mut direction) = direction {
                        handle_water_grain(&mut transform, &mut grid_position, &grid_data, &mut *direction)
                    }
                }
            }
        }
    }
}

fn update_grid_data(mut query: Query<(&GrainType, &mut GridPosition)>, mut grid_data: ResMut<Grid>) {
    for (grain_type, mut pos) in query.iter_mut() {
        // Clear the previous position from the grid
        if let (Some(prev_x), Some(prev_y)) = (pos.prev_x, pos.prev_y) {
            if prev_x >= 0 && prev_y >= 0 && prev_x < GAME_RESOLUTION_X as i32 && prev_y < GAME_RESOLUTION_Y as i32 {
                grid_data.data[prev_y as usize][prev_x as usize] = None;
            }
        }

        // Boundary checks for current positions
        if pos.current_x >= 0
            && pos.current_x < GAME_RESOLUTION_X as i32
            && pos.current_y >= 0
            && pos.current_y < GAME_RESOLUTION_Y as i32
        {
            // Update the grid with the new position
            grid_data.set(pos.current_x as usize, pos.current_y as usize, *grain_type);
        }

        // Update prev for next frame
        pos.prev_x = Some(pos.current_x);
        pos.prev_y = Some(pos.current_y);
    }
}
