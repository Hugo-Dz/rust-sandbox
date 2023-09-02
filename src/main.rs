use bevy::prelude::*;
use bevy::window::PresentMode;

#[derive(Component)]
struct Cube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lines".into(),
                resolution: (640., 640.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup_camera, add_player))
        .add_systems(Update, greet)
        .run();
}

// These systems will only run once since they are registered on in Startup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn add_player(mut commands: Commands) {
    // Spawn a row (entity) with this set of components
    commands.spawn((Player, Name("Hugo".to_string())));
}



#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

fn greet(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("Hello, {}", name.0);
    }
}