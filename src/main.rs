//! Ray-tracer written in Rust.
#![feature(io, os, core, std_misc, path)]
extern crate image;

use std::old_io::File;
use std::num::Float;

use image::GenericImage;

use self::vector::Vector;
use self::shape::sphere::Sphere;
use self::shape::Shape;

pub mod vector;
pub mod shape;

#[allow(dead_code)]
fn main() {
    let size = (80, 60);
    let (x_size, y_size) = size;
    let mut buffer = image::ImageBuffer::new(x_size, y_size);
    let camera = Vector::new(0.0f32, 0.0, 0.0);
    let standoff = -100.0f32;
    let div = y_size as f32 / (standoff * Float::tan(60.0));

    let center = Vector::new(25.0, 0.0, -300.0);
    let spheres = vec![
            Sphere::new(20.0, center, (255, 0, 0)),
            Sphere::new(10.0, Vector::new(-25.0, 3.0, -280.0), (0, 255, 0)),
            Sphere::new(15.0, Vector::new(-2.0, 25.0, -320.0), (0, 0, 255)),
    ];

    for x in (0..x_size) {
        for y in (0..y_size) {
            let start = Vector::new((x as f32 - x_size as f32 / 2.0) / div, (y as f32 - y_size as f32 / 2.0) / div, standoff);
            let ray = Vector::new(start.x - camera.x, start.y - camera.y, start.z - camera.z);
            let amp = 1.0 / ray.mag();
            let norm_ray = Vector::new(ray.x * amp, ray.y * amp, ray.z * amp);
            for sphere in spheres.iter() {
                match sphere.intersect(norm_ray.clone()) {
                    Some(_) => {
                        let (r, g, b) = sphere.get_color(norm_ray.clone());
                        buffer.put_pixel(x, y, image::Rgb([r, g, b]))
                    },
                    None => {}
                }
            }
        }
    }

    let output = File::create(&Path::new("ray_trace.png"));
    match output {
        Err(why) => {
            println!("Failed {}", why);
            std::os::set_exit_status(1);
        },
        Ok(output) => {
            let _ = image::ImageRgb8(buffer).save(output, image::PNG);
        }
    }
}
