use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use std::f32::consts::PI;

use crate::player::Player;
pub struct CameraPlugin;

#[derive(Component)]
pub struct OrbitCamera {
    pub radius: f32,
    pub sensitivity: f32,
    pub target: Vec3,
    pub orbit_button: MouseButton,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, orbit_mouse);
        app.add_systems(Update, sync_camera_with_player);
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
                y: 2.5,
                z: 0.0,
            },
            radius: 10.0,
            sensitivity: 300.0,
            orbit_button: MouseButton::Left,
        },
    );

    commands.spawn(camera);
}

fn sync_camera_with_player(
    player_query: Query<&Transform, With<Player>>,
    mut cam_query: Query<(&mut OrbitCamera, &mut Transform), Without<Player>>,
    mouse: Res<Input<MouseButton>>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(error) => Err(format!("Error getting player transform: {}", error)).unwrap(),
    };

    let camera = match cam_query.get_single_mut() {
        Ok(camera) => camera,
        Err(error) => Err(format!("Error getting player camera: {}", error)).unwrap(),
    };

    let mut orbit_camera = camera.0;
    let mut camera_transform = camera.1;

    if mouse.just_pressed(orbit_camera.orbit_button) {
        camera_transform.translation = orbit_camera.target;
    }

    orbit_camera.target = player_transform.translation;

    if !mouse.pressed(orbit_camera.orbit_button) {
        let rot_matrix = Mat3::from_quat(camera_transform.rotation);

        camera_transform.translation =
            orbit_camera.target + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbit_camera.radius));
    }
}

fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&OrbitCamera, &mut Transform), With<OrbitCamera>>,
    mouse: Res<Input<MouseButton>>,
    mut mouse_evr: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.read() {
        rotation = ev.delta
    }

    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    if !mouse.pressed(cam.orbit_button) {
        return;
    }

    rotation *= cam.sensitivity * time.delta_seconds();

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width() * std::f32::consts::PI;
            delta
        };

        let delta_y = rotation.y / window.height() * PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation; // rotate around global y axis

        // Calculate the new rotation without applying it to the camera yet
        let new_rotation = cam_transform.rotation * pitch;

        // check if new rotation will cause camera to go beyond the 180 degree vertical bounds
        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }

    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation = cam.target + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));
}
