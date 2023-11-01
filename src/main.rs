mod blood;
mod bone;
mod grain;
mod grid;
mod setup;
mod utils;

use crate::blood::shade_blood;
use crate::grain::{add_grain, update_grain};
use crate::grid::{update_grid_data, Grid};
use crate::setup::setup;
use crate::utils::constants::{TICK_RATE, WINDOW_SIZE};
use crate::utils::tick::TickCounter;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_pixel_camera::PixelCameraPlugin;

/*

    🦀 Main function, used to:
    - Add Ressources -> Resources are global data we can use later across Systems, independently of entities. Learn more: https://bevy-cheatbook.github.io/programming/res.html
    - Register Systems -> Think Bevy ECS Systems like a database, Systems are a way to "query" our data to implement game logic. Learn more: https://bevy-cheatbook.github.io/programming/systems.html
    - Run our app

*/

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
        .insert_resource(ClearColor(Color::hex("221e22").unwrap()))
        .insert_resource(TickCounter {
            count: 0,
            tick_rate: TICK_RATE,
        })
        .insert_resource(Grid::new())
        .add_systems(Startup, setup)
        .add_systems(Update, add_grain)
        .add_systems(Update, update_grain)
        .add_systems(Update, update_grid_data)
        .add_systems(Update, shade_blood)
        .run();
}
