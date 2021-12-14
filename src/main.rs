extern crate kiss3d;
extern crate nalgebra as na;
extern crate noise;

use crate::utils::*;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, Translation3};

use noise::*;
use std::{path::Path, vec::Vec};

mod map;

fn remove_face(f: &mut Vec<Point3<u16>>) {
    if f.len() > 10 {
        f.remove(5);
        f.remove(5);
        f.remove(5);
    }
}

fn add_water(window: &mut Window, i: f32, b: f32) -> kiss3d::scene::SceneNode {
    let mut c = window.add_cube(1.0, 1.0, 1.0);
    c.set_local_translation(Translation3::new(i, -1.0, b));
    c.modify_faces(&mut remove_face);
    c.set_color(0.0, 0.47, 0.75);
    return c;
}

fn main() {
    let mut window = Window::new("MineRS");
    let mut vec: Vec<kiss3d::scene::SceneNode> = Vec::new();
    let path_stone = Path::new("./assets/cube.jpg");
    let path_dirt = Path::new("./assets/cube3.jpg");
    let path_sand = Path::new("./assets/sand.jpg");

    let map = map::generate_map();

    ImageRenderer::new()
        .set_gradient(ColorGradient::new().build_terrain_gradient())
        .render(&map)
        .write_to_file("unscaledFinalPlanet.png");
    for i in 0..50 {
        for b in 0..50 {
            let mut map_off = map.get_value(i, b);
            map_off = map_off * 20.0;
            let mut c = window.add_cube(1.0, 1.0, 1.0);
            c.set_local_translation(Translation3::new(
                i as f32,
                (map_off as i32) as f32,
                b as f32,
            ));
            if (map_off as i32) < -4 {
                c.set_texture_from_file(path_dirt, "dirt");
                add_water(&mut window, i as f32, b as f32);
            } else if (map_off as i32) < 1 {
                c.set_texture_from_file(path_sand, "sand");
            } else {
                c.set_texture_from_file(path_stone, "stone");
            }
            c.modify_faces(&mut remove_face);
            c.modify_faces(&mut remove_face);
            for z in (0..12).rev() {
                let mut d = window.add_cube(1.0, 1.0, 1.0);
                d.set_local_translation(Translation3::new(
                    i as f32,
                    ((map_off as i32) - z) as f32,
                    b as f32,
                ));
                if (map_off as i32) < -4 {
                    d.set_texture_from_file(path_dirt, "dirt");
                } else if (map_off as i32) < 1 {
                    d.set_texture_from_file(path_sand, "sand");
                } else {
                    d.set_texture_from_file(path_stone, "stone");
                }
                vec.push(d);
            }
            vec.push(c);
        }
    }
    window.set_background_color(0.53, 0.81, 0.92);

    window.set_light(Light::StickToCamera);

    while window.render() {}
}
