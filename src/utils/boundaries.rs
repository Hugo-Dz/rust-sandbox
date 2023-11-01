use crate::grid::GridPosition;



pub fn is_within_boundaries(grid_position: &GridPosition, res_x: i32, res_y: i32) -> bool {
    grid_position.current_x >= 1
        && grid_position.current_x < res_x - 1
        && grid_position.current_y >= 0
        && grid_position.current_y < res_y - 1
}
