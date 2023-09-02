use bevy::prelude::*;
use bevy::window::PresentMode;

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
        //.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup, add_player))
        .add_systems(Update, move_player)
        .run();
}

// These systems will only run once since they are registered on in Startup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("player.png");
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    };

    // Spawn a row (entity) with this set of components
    commands.spawn((Player, sprite_bundle));
}

#[derive(Component)]
struct Player;

fn move_player(time: Res<Time>, input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &Player)>) {
    for (mut transform, _) in query.iter_mut() {
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}
