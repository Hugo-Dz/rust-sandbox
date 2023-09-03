mod utils;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use rand::Rng;
use bevy::math::Vec2;
use bevy::sprite::Anchor;
use crate::utils::remap::remap_cursor_position;

const GAME_RESOLUTION: [i32; 2] = [80, 60];
const WINDOW_SIZE: [f32; 2] = [640.0, 480.0];

#[derive(Resource)]
struct TickCounter {
    count: u32,
    tick_rate: u32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy".into(),
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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(TickCounter { count: 0, tick_rate: 1 })
        .add_systems(Startup, setup)
        .add_systems(Update, (add_grain, drop_grain))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(GAME_RESOLUTION[0], GAME_RESOLUTION[1], true));
}

#[derive(Component)]
struct Grain;

#[derive(Component)]
enum Type {
    Sand,
    _Rock,
    _Water,
}

fn add_grain(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    query: Query<&Window>,
) {
    let sand_textures: [Handle<Image>; 3] = [
        asset_server.load("sand1.png"),
        asset_server.load("sand2.png"),
        asset_server.load("sand3.png"),
    ];
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..sand_textures.len());
    let texture = sand_textures[random_index].clone();

    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        texture,
        ..default()
    };

    if let Some(position) = query.single().cursor_position() {
        if input.pressed(MouseButton::Left) {
            let remaped_cursor_pos = remap_cursor_position(position, WINDOW_SIZE, GAME_RESOLUTION);
            // Add a row (entity) with this set of components
            commands.spawn((Grain, sprite_bundle, Type::Sand)).insert(Transform {
                translation: Vec3::new(remaped_cursor_pos.x.round() - (GAME_RESOLUTION[0] as f32 / 2.0), -(remaped_cursor_pos.y.round() - (GAME_RESOLUTION[1] as f32 / 2.0)), 0.0),
                ..default()
            });
        }
    }
}

fn drop_grain(mut query: Query<(&mut Transform, &Grain)>, mut tick_counter: ResMut<TickCounter>) {
    tick_counter.count += 1;

    if tick_counter.count >= tick_counter.tick_rate {

        tick_counter.count = 0;

        for (mut transform, _) in query.iter_mut() {
            if transform.translation.y >= -(GAME_RESOLUTION[1] as f32 / 2.0 - 2.0) {
                transform.translation.y -= 1.0;
            }
        }
    }
}
