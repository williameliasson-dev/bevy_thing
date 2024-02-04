use bevy::{prelude::*, render::camera};

use crate::camera::OrbitCamera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movements);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Speed {
    value: f32,
}

fn player_movements(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    mut camera_query: Query<(&Transform, &mut OrbitCamera), (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_query.iter_mut() {
        let (camera, mut orbit_camera) = match camera_query.get_single_mut() {
            Ok(camera) => camera,
            Err(error) => Err(format!("Error getting camera: {}", error)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            direction += camera.forward();
        }

        if keys.pressed(KeyCode::S) {
            direction += camera.back()
        }

        if keys.pressed(KeyCode::A) {
            direction += camera.left()
        }

        if keys.pressed(KeyCode::D) {
            direction += camera.right()
        }

        direction.y = 0.0;

        let movement = direction.normalize_or_zero() * player_speed.value * time.delta_seconds();

        player_transform.translation += movement;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Speed { value: 3.0 },
        Player,
    );

    commands.spawn(player);
}
