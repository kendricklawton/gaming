use bevy::prelude::*;
use core::CorePlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            CorePlugin,
            CameraPlugin,
            PlayerPlugin,
        ))
        .run();
}
