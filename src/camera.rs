use bevy::prelude::*;

use crate::FOREGROUND;

// Useful for marking the "main" camera if we have many
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct BackgroundCamera;

/// Initialize camera
/// 
/// 
pub fn initialize_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle::default(),
        FOREGROUND,
        MainCamera
    ));
    
    // commands.spawn((
    //     Camera2dBundle::default(),
    //     BACKGROUND,
    //     BackgroundCamera
    // ));
}

/// Zoom control system
/// 
/// 
pub fn zoom_control_system(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut projection = camera_query.single_mut();
    
    if input.pressed(KeyCode::Minus) {
        projection.scale += 0.2;
    }
    
    // In a normal keyboard, the equal is also the plus sign
    if input.pressed(KeyCode::Equal) {
        projection.scale -= 0.2;
    }
    
    projection.scale = projection.scale.clamp(0.2, 5.);
}

/// Get world coordinates of the mouse
/// 
/// This is useful for placing blocks or breaking them.
pub fn mouse_coordinates(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        info!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
