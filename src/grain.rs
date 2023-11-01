use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;
use crate::grid::{Grid, GridPosition};
use crate::utils::remap::remap_cursor_position;
use crate::blood::handle_blood_grain;
use crate::bone::handle_bone_grain;
use crate::utils::constants::{GAME_RESOLUTION_X, GAME_RESOLUTION_Y, WINDOW_SIZE};
use crate::utils::tick::TickCounter;

#[derive(Component)]
pub struct Grain;

#[derive(Component)]
pub struct Lifetime(u32);

#[derive(Component, Clone, Copy, Debug)]
pub enum GrainType {
    Bone,
    Blood,
}

#[derive(Component, PartialEq)]
pub enum Direction {
    Right,
    Left,
}

pub fn update_grain(
    grid_data: Res<Grid>,
    mut query: Query<(
        &mut Transform,
        &mut GridPosition,
        &GrainType,
        &mut Lifetime,
        Option<&mut Direction>,
    )>,
    mut tick_counter: ResMut<TickCounter>,
) {
    tick_counter.count += 1;
    if tick_counter.count >= tick_counter.tick_rate {
        tick_counter.count = 0;
        for (mut transform, mut grid_position, grain_type, mut lifetime, direction) in query.iter_mut() {
            lifetime.0 += 1;
            match grain_type {
                GrainType::Bone => handle_bone_grain(&grid_position, &grid_data),
                GrainType::Blood => {
                    if let Some(mut dir) = direction {
                        handle_blood_grain(&mut transform, &mut grid_position, &grid_data, &mut dir, lifetime.0);
                    }
                }
            }
        }
    }
}

pub fn add_grain(
    mut commands: Commands,
    query: Query<&Window>,
    asset_server: Res<AssetServer>,
    input: Res<Input<MouseButton>>
) {

    if let Some(position) = query.single().cursor_position() {

        let remaped_cursor_pos = remap_cursor_position(position, WINDOW_SIZE, [GAME_RESOLUTION_X, GAME_RESOLUTION_Y]);
        let mut rng = rand::thread_rng();

        if input.pressed(MouseButton::Left) {

            // Create a bone sprite texture
            let bone_textures: [&str; 3] = [
                "bone_1.png",
                "bone_2.png",
                "bone_3.png"
            ];
            let random_index = rng.gen_range(0..bone_textures.len());
        
            let bone_sprite_bundle = SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture: asset_server.load(bone_textures[random_index]),
                ..default()
            };

            // Add a data row (entity) with it's set of components
            commands
                .spawn((
                    Grain,
                    bone_sprite_bundle,
                    GrainType::Bone,
                    GridPosition {
                        current_x: remaped_cursor_pos.x.round() as i32,
                        current_y: remaped_cursor_pos.y.round() as i32,
                        prev_x: None,
                        prev_y: None,
                    },
                ))
                .insert(Transform {
                    translation: Vec3::new(
                        remaped_cursor_pos.x.round() - (GAME_RESOLUTION_X as f32 / 2.0),
                        -(remaped_cursor_pos.y.round() - (GAME_RESOLUTION_Y as f32 / 2.0)),
                        0.0,
                    ),
                    ..default()
                });
        }

        if input.pressed(MouseButton::Right) {

            // Creat a blood sprite texture
            let blood_sprite_bundle = SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture: asset_server.load("blood_2.png"),
                ..default()
            };

            // Pick a random direction to slide toward
            let random_number = rng.gen::<f32>();

            let random_direction = if random_number < 0.5 {
                Direction::Left
            } else {
                Direction::Right
            };

            // Add a data row (entity) with it's set of components
            commands
                .spawn((
                    Grain,
                    Lifetime(0),
                    blood_sprite_bundle,
                    GrainType::Blood,
                    random_direction,
                    GridPosition {
                        current_x: remaped_cursor_pos.x.round() as i32,
                        current_y: remaped_cursor_pos.y.round() as i32,
                        prev_x: None,
                        prev_y: None,
                    }
                ))
                .insert(Transform {
                    translation: Vec3::new(
                        remaped_cursor_pos.x.round() - (GAME_RESOLUTION_X as f32 / 2.0),
                        -(remaped_cursor_pos.y.round() - (GAME_RESOLUTION_Y as f32 / 2.0)),
                        0.0,
                    ),
                    ..default()
                });
        }
    }
}
