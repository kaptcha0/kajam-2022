use bevy::{prelude::*, render::camera::ScalingMode};

pub struct CameraBundle;

impl Plugin for CameraBundle {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera).insert(MainCamera);
}
