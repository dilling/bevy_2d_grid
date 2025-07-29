use bevy::{prelude::*, render::view::RenderLayers};
use bevy_2d_grid::{InfiniteGrid2DBundle, InfiniteGrid2DPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InfiniteGrid2DPlugin))
        .add_systems(Startup, setup_system)
        .add_systems(Update, toggle_layers)
        .run();
}

fn setup_system(mut commands: Commands) {
    // Spawn the infinite 2D grid on render layer 1
    commands.spawn((InfiniteGrid2DBundle::default(), RenderLayers::layer(1)));

    // Spawn a 2D camera on render layer 1
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 10.0),
        RenderLayers::layer(1),
    ));

    // Add some 2D sprites for reference on different layers
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 1.0),
        RenderLayers::layer(1), // Same layer as grid
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(-150.0, 50.0, 1.0),
        RenderLayers::layer(0), // Different layer - won't show initially
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.8),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 1.0),
        RenderLayers::layer(1), // Same layer as grid
    ));
}

fn toggle_layers(mut q: Query<&mut RenderLayers, With<Camera>>, input: Res<ButtonInput<KeyCode>>) {
    for mut render_layers in &mut q {
        if input.just_pressed(KeyCode::KeyT) {
            if render_layers.intersects(&RenderLayers::layer(1)) {
                *render_layers = RenderLayers::layer(0);
            } else {
                *render_layers = RenderLayers::layer(1);
            }
        }
    }
}
