use bevy::prelude::*;

use crate::camera::initialize_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    /// Build
    /// 
    /// 
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
    }
}
