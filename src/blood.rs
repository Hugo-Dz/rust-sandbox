use bevy::prelude::*;
use crate::utils::constants::{GAME_RESOLUTION_X, GAME_RESOLUTION_Y, BLOOD_SAFE_TIME};
use crate::utils::boundaries::is_within_boundaries;
use crate::grid::{Grid, GridPosition};
use crate::grain::{GrainType, Direction};

fn move_grain(transform: &mut Transform, grid_position: &mut GridPosition, x: i32, y: i32) {
    transform.translation.x += x as f32;
    transform.translation.y -= y as f32;
    grid_position.prev_x = Some(grid_position.current_x);
    grid_position.prev_y = Some(grid_position.current_y);
    grid_position.current_x += x;
    grid_position.current_y += y;
}

pub fn handle_blood_grain(
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

pub fn shade_blood(
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
