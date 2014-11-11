extern crate image;

use std::io::File;
use std::num::FloatMath;

use image::GenericImage;

use self::point::Point;
use self::shape::sphere::Sphere;

pub mod point;
pub mod shape;

#[allow(dead_code)]
fn main() {
    let size = (80, 60);
    let (x_size, y_size) = size;
    let mut buffer = image::ImageBuf::new(x_size, y_size);
    let camera = Point::new(0.0f32, 0.0, 0.0);
    let standoff = -100.0;
    let div = y_size as f32 / (standoff as f32 * FloatMath::tan(60.0));

    let pixel = image::Luma(255u8);
    let center = Point::new(25.0f32, 0.0, -300.0);
    let sphere = Sphere::new(20.0f32, center);

    for x in range(0, x_size) {
        for y in range(0, y_size) {
            let start = Point::new((x as f32 - x_size as f32 / 2.0) / div, (y as f32 - y_size as f32 / 2.0) / div, standoff);
            let ray = Point::new(start.x - camera.x, start.y - camera.y, start.z - camera.z);
            let amp = 1.0 / ray.mag();
            let norm_ray = Point::new(ray.x * amp, ray.y * amp, ray.z * amp);
            match sphere.intersect(norm_ray) {
                Some(_) => buffer.put_pixel(x, y, pixel),
                None => {}
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
            let _ = image::ImageLuma8(buffer).save(output, image::PNG);
        }
    }
}
