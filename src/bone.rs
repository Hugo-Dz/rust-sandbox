use crate::grid::{Grid, GridPosition};
use crate::utils::constants::{GAME_RESOLUTION_X, GAME_RESOLUTION_Y};
use crate::utils::boundaries::is_within_boundaries;
use crate::grain::GrainType;

pub fn handle_bone_grain(grid_position: &GridPosition, grid_data: &Grid) {
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
