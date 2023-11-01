pub const GAME_RESOLUTION_X: usize = 80; // Change the game resolution for finer grains, eq. 160x120
pub const GAME_RESOLUTION_Y: usize = 60;
pub const WINDOW_SIZE: [f32; 2] = [640.0, 480.0];
pub const TICK_RATE: u32 = 1; // Speed of the simulation, higher number is slower
pub const BLOOD_SAFE_TIME: u32 = 200; // Stop blood interaction after this time to avoid infinite back and forth movement