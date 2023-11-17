use crate::utils::constants::{GAME_RESOLUTION_X, GAME_RESOLUTION_Y};
use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;

/*

    🦀 Add a new entity with the camera components

*/

pub fn setup(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(
        GAME_RESOLUTION_X as i32,
        GAME_RESOLUTION_Y as i32,
        true,
    ));
}
