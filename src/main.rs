use bevy::prelude::*;
use bevy::window::PresentMode;

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy)]
struct GridPosition(i32, i32);

#[derive(Resource)]
struct GrainDropCounter {
    count: u32,
}

const GRAIN_SIZE: f32 = 10.0;

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

#[derive(Component)]
enum Type {
    Sand,
    _Rock,
    _Water
}


fn add_grain(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    query: Query<&Window>,
) {
    let texture = asset_server.load("pixel.png");
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(GRAIN_SIZE, GRAIN_SIZE)),
            ..default()
        },
        texture,
        ..default()
    };

    if let Some(position) = query.single().cursor_position() {
        
        if input.pressed(MouseButton::Left) {
            let grid_x = ((position.x - 320.0) / GRAIN_SIZE).floor() as i32;
            let grid_y = (-(position.y - 240.0) / GRAIN_SIZE).floor() as i32;
            let grid_position = GridPosition(grid_x, grid_y);
            // Add a row (entity) with this set of components
            commands.spawn((Grain, sprite_bundle, grid_position, Type::Sand)).insert(Transform {
                translation: Vec3::new(grid_position.0 as f32 * GRAIN_SIZE + (GRAIN_SIZE / 2.0), grid_position.1 as f32 * GRAIN_SIZE + (GRAIN_SIZE / 2.0), 0.0),
                ..default()
            });
        }
    }
}

fn drop_grain(mut query: Query<(&mut GridPosition, &Grain)>, mut counter: ResMut<GrainDropCounter>) {
    counter.count += 1;

    if counter.count >= 5 {
        for (mut grid_position, _) in query.iter_mut() {
            if grid_position.1 > -(240.0 / GRAIN_SIZE) as i32 {
                grid_position.1 -= 1;
            }
        }
        counter.count = 0;
    }
}

fn update_transform(mut query: Query<(&mut Transform, &GridPosition, &Grain)>) {
    for (mut transform, grid_position, _) in query.iter_mut() {
        transform.translation = Vec3::new(grid_position.0 as f32 * GRAIN_SIZE + (GRAIN_SIZE / 2.0), grid_position.1 as f32 * GRAIN_SIZE + (GRAIN_SIZE / 2.0), 0.0);
    }
}
