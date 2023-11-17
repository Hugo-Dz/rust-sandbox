use crate::grain::GrainType;
use crate::utils::constants::{GAME_RESOLUTION_X, GAME_RESOLUTION_Y};
use bevy::prelude::*;

/*

    🦀 Resources and Components. Learn more: https://bevy-cheatbook.github.io/programming/res.html

*/

#[derive(Resource)]
pub struct Grid {
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
pub struct GridPosition {
    pub current_x: i32,
    pub current_y: i32,
    pub prev_x: Option<i32>,
    pub prev_y: Option<i32>,
}

/*

    🦀 Systems. Learn more: https://bevy-cheatbook.github.io/programming/systems.html

*/

pub fn update_grid_data(mut query: Query<(&GrainType, &mut GridPosition)>, mut grid_data: ResMut<Grid>) {
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
            grid_data.set(
                grid_position.current_x as usize,
                grid_position.current_y as usize,
                *grain_type,
            );
        }

        // Update prev for next frame
        grid_position.prev_x = Some(grid_position.current_x);
        grid_position.prev_y = Some(grid_position.current_y);
    }
}
