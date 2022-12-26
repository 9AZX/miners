use bevy::app::App;
use bevy::DefaultPlugins;
use crate::map::map_generator::generate_map;

pub mod map;
pub mod camera;

use crate::map::map_plugin::MapPlugin;

fn main() {
    generate_map(1234);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(camera::CameraPlugin)
        .run();
}
