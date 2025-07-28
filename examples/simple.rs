use bevy::{input::mouse::{MouseWheel, MouseMotion}, prelude::*};
use bevy_infinite_grid::{InfiniteGrid2DBundle, InfiniteGrid2DPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InfiniteGrid2DPlugin))
        .add_systems(Startup, setup_system)
        .add_systems(Update, (camera_movement, camera_zoom))
        .run();
}

fn setup_system(mut commands: Commands) {
    // Spawn the infinite 2D grid
    commands.spawn(InfiniteGrid2DBundle::default());

    // Spawn a 2D camera
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 10.0),
        CameraMovement::default(),
    ));

    // Add some 2D sprites for reference
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(150.0, 150.0, 1.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, 200.0, 1.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.8),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(30.0, -230.0, 1.0),
    ));
}

#[derive(Component)]
struct CameraMovement {
    speed: f32,
}

impl Default for CameraMovement {
    fn default() -> Self {
        Self { speed: 200.0 }
    }
}

fn camera_movement(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &CameraMovement, &Projection), With<Camera2d>>,
) {
    let Ok((mut transform, movement, projection)) = camera_query.single_mut() else {
        return;
    };
    let dt = time.delta_secs();
    
    // Handle click-to-pan with mouse
    if mouse_input.pressed(MouseButton::Left) {
        for motion in mouse_motion.read() {
            // Convert mouse delta to world space movement
            let mouse_delta = motion.delta;
            
            // Scale the movement by the camera's orthographic scale for proper panning
            let scale = if let Projection::Orthographic(ortho) = projection {
                ortho.scale
            } else {
                1.0
            };
            
            // Pan in opposite direction of mouse movement (natural feel)
            transform.translation.x -= mouse_delta.x * scale;
            transform.translation.y += mouse_delta.y * scale; // Y is flipped in screen space
        }
    }
    
    // Handle keyboard movement
    let mut direction = Vec2::ZERO;
    
    if key_input.pressed(KeyCode::KeyW) || key_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if key_input.pressed(KeyCode::KeyS) || key_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if key_input.pressed(KeyCode::KeyA) || key_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if key_input.pressed(KeyCode::KeyD) || key_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    
    if direction != Vec2::ZERO {
        direction = direction.normalize();
        let movement_delta = direction * movement.speed * dt;
        transform.translation.x += movement_delta.x;
        transform.translation.y += movement_delta.y;
    }
}

fn camera_zoom(
    key_input: Res<ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Projection, With<Camera2d>>,
) {
    if let Ok(mut projection) = camera_query.single_mut() {
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            let zoom_speed = 0.1;
            
            // Keyboard zoom controls
            if key_input.just_pressed(KeyCode::Equal) || key_input.just_pressed(KeyCode::NumpadAdd) {
                // Zoom in - decrease scale (makes things appear larger)
                ortho.scale *= 1.0 - zoom_speed;
                ortho.scale = ortho.scale.max(0.1); // Cap max zoom
            }
            
            if key_input.just_pressed(KeyCode::Minus) || key_input.just_pressed(KeyCode::NumpadSubtract) {
                // Zoom out - increase scale (makes things appear smaller)
                ortho.scale *= 1.0 + zoom_speed;
                ortho.scale = ortho.scale.min(10.0); // Cap min zoom
            }
            
            // Scroll wheel zoom controls
            for scroll in scroll_events.read() {
                let scroll_zoom_speed = 0.05; // Smaller steps for smoother scrolling
                
                if scroll.y > 0.0 {
                    // Scroll up - zoom in
                    ortho.scale *= 1.0 - scroll_zoom_speed;
                    ortho.scale = ortho.scale.max(0.1);
                } else if scroll.y < 0.0 {
                    // Scroll down - zoom out
                    ortho.scale *= 1.0 + scroll_zoom_speed;
                    ortho.scale = ortho.scale.min(10.0);
                }
            }
        }
    }
}
