use bevy::prelude::*;

#[derive(Resource)]
pub struct TickCounter {
    pub count: u32,
    pub tick_rate: u32,
}
