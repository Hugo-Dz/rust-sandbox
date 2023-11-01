use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;
use crate::utils::constants::{GAME_RESOLUTION_X, GAME_RESOLUTION_Y};

pub fn setup(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(
        GAME_RESOLUTION_X as i32,
        GAME_RESOLUTION_Y as i32,
        true,
    ));
}
