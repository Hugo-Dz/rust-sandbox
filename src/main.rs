use bevy::prelude::*;
use bevy::window::PresentMode;
use rand::Rng;
use std::time::Instant;

#[derive(Component)]
struct Cube;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy".into(),
                        resolution: (640., 480.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, (add_grain, fly_grain))
        .run();
}

// These systems will only run once since they are registered on in Startup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Grain;

#[derive(Component)]
struct SpawnedTime(Instant);

fn add_grain(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    query: Query<&Window>,
    time: Res<Time>
) {
    let texture = asset_server.load("pixel.png");
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        texture,
        ..default()
    };

    if let Some(position) = query.single().cursor_position() {
        // Spawn a row (entity) with this set of components
        if input.pressed(MouseButton::Left) {
            commands.spawn((Grain, sprite_bundle, SpawnedTime(time.last_update().unwrap()))).insert(Transform {
                translation: Vec3::new(position.x - 320.0, -(position.y - 240.0), 0.0),
                ..default()
            });
        }
    }
}

fn fly_grain(mut query: Query<(&mut Transform, &Grain, &SpawnedTime)>, time: Res<Time>) {
    for (mut transform, _, spawned_time) in query.iter_mut() {

        let time_since_spawn = Instant::now().duration_since(spawned_time.0).as_secs_f32();

        let mut rng = rand::thread_rng();
        let rand_x: f32 = rng.gen_range(-150.0..150.0) + time_since_spawn;
        transform.translation.y -= 100.0 * time.delta_seconds();
        transform.translation.x -= rand_x * time.delta_seconds();
    }
}
