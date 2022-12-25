use bevy::app::App;
use bevy::DefaultPlugins;

pub mod map;
pub mod camera;

use crate::map::map_plugin::MapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(camera::CameraPlugin)
        .run();
}
