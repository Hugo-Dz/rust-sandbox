use bevy::prelude::*;
use bevy::window::PresentMode;

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy)]
struct GridPosition(i32, i32);

#[derive(Resource)]
struct GrainDropCounter {
    count: u32,
}

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
        .insert_resource(GrainDropCounter { count: 0 })
        .add_systems(Startup, setup)
        .add_systems(Update, (add_grain, drop_grain, update_transform))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Grain;

fn add_grain(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    query: Query<&Window>,
) {
    let texture = asset_server.load("pixel.png");
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        texture,
        ..default()
    };

    if let Some(position) = query.single().cursor_position() {
        // Spawn a grain with this set of components
        if input.pressed(MouseButton::Left) {
            let grid_x = ((position.x - 320.0) / 10.0).floor() as i32;
            let grid_y = (-(position.y - 240.0) / 10.0).floor() as i32;
            let grid_position = GridPosition(grid_x, grid_y);
            commands.spawn((Grain, sprite_bundle, grid_position)).insert(Transform {
                translation: Vec3::new(grid_position.0 as f32 * 10.0, grid_position.1 as f32 * 10.0, 0.0),
                ..default()
            });
        }
    }
}

fn drop_grain(mut query: Query<(&mut GridPosition, &Grain)>, mut counter: ResMut<GrainDropCounter>) {
    counter.count += 1;

    if counter.count >= 5 {
        // Update position every 10 frames
        for (mut grid_position, _) in query.iter_mut() {
            if grid_position.1 > (-240 + 5) / 10 {
                // Check if above ground
                grid_position.1 -= 1;
            }
        }
        counter.count = 0; // Reset the counter
    }
}

fn update_transform(mut query: Query<(&mut Transform, &GridPosition, &Grain)>) {
    for (mut transform, grid_position, _) in query.iter_mut() {
        transform.translation = Vec3::new(grid_position.0 as f32 * 10.0, grid_position.1 as f32 * 10.0, 0.0);
    }
}
