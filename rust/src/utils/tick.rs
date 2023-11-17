use bevy::prelude::*;

/*

    🦀 Resources and Components. Learn more: https://bevy-cheatbook.github.io/programming/res.html

*/

#[derive(Resource)]
pub struct TickCounter {
    pub count: u32,
    pub tick_rate: u32,
}
