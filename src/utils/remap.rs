use bevy::math::Vec2;

pub fn remap_cursor_position(pos: Vec2, from: [f32; 2], to: [i32; 2]) -> Vec2 {
    let new_x = (pos.x / from[0]) * to[0] as f32;
    let new_y = (pos.y / from[1]) * to[1] as f32;
    Vec2::new(new_x, new_y)
}