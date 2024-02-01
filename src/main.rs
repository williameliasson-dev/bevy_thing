use bevy::prelude::*;

mod camera;
mod scenes;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (scenes::setup_test_scene, camera::spawn_camera))
        .run();
}
