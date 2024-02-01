use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use scenes::ScenePlugin;

mod camera;
mod player;
mod scenes;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, ScenePlugin))
        .run();
}
