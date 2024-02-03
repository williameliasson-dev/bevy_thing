use bevy::{input::mouse::MouseMotion, prelude::*, render::camera};
use std::f32::consts::PI;
pub struct CameraPlugin;

#[derive(Component)]
struct OrbitCamera {
    radius: f32,
    sensitivity: f32,
    target: Vec3,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, update_camera_rotation);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCamera {
            target: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 10.0,
            sensitivity: 0.0002,
        },
    );

    commands.spawn(camera);
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn calculate_camera_position(target: &Vec3, radius: f32, rotation_y: f32, rotation_x: f32) -> Vec3 {
    let radians_x = degrees_to_radians(rotation_x);
    let radians_y = degrees_to_radians(rotation_y);

    Vec3 {
        x: target.x + radius * radians_y.cos() * radians_x.cos(),
        y: target.y + radius * radians_x.sin(),
        z: target.z + radius * radians_y.sin() * radians_x.cos(),
    }
}

fn update_camera_rotation(
    mut camera_query: Query<(&mut Transform, &OrbitCamera), With<Camera>>,
    mut mouse_er: EventReader<MouseMotion>,
) {
    let mut cameraRotX: f32 = 0.0;
    let mut cameraRotY: f32 = 0.0;

    for mouse_ev in mouse_er.read() {
        cameraRotX += mouse_ev.delta.y;
        cameraRotY += mouse_ev.delta.x;
    }

    cameraRotX.clamp(-89.0, 89.0);

    let camera = match camera_query.get_single_mut() {
        Ok(camera) => camera,
        Err(error) => Err(format!("Error retriving camera: {}", error)).unwrap(),
    };

    let orbit_camera = camera.1;
    let mut camera_transform = camera.0;
}
