use bevy::prelude::*;
use victimless_camera::prelude::*;

const RES_WIDTH: u32 = 854;
const RES_HEIGHT: u32 = 480;

pub struct CameraPlugin;

#[derive(Component, Default)]
pub struct OuterCamera;

#[derive(Component, Default)]
pub struct Canvas;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_plugins(PixelCameraPlugin::<MainCamera, OuterCamera, Canvas>::new(
                RES_WIDTH, RES_HEIGHT,
            ))
            .add_systems(Update, position_camera);
    }
}

fn position_camera(mut camera_query: Query<&mut Transform, With<MainCamera>>) {
    for mut transform in &mut camera_query {
        transform.translation = Vec3::new(10.0, 10.0, 10.0);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
