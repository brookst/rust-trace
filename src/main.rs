extern crate image;

use std::io::File;
use std::num::FloatMath;

use image::GenericImage;

use self::point::Point;
mod point;

fn sphere_intersect(ray: Point, center: Point, radius: f32) -> Option<f32> {
    let b = -(ray.x * center.x + ray.y * center.y + ray.z * center.z);
    let det = b * b + radius * radius - center.mag2();
    if ray.x == 0.0 && ray.y == 0.0 {
        println!("ray: {}", ray);
        println!("center: {}", center);
        println!("radius: {}", radius);
        println!("det: {}", det);
    }
    if det < 0.0 {
        None
    } else {
        Some(b - det)
    }
}

fn main() {
    let size = (80, 60);
    let (x_size, y_size) = size;
    let mut buffer = image::ImageBuf::new(x_size, y_size);
    let camera = Point{x: 0.0, y: 0.0, z: 0.0};
    let standoff = -100i;
    let div = y_size as f32 / (standoff as f32 * FloatMath::tan(60.0));

    let pixel = image::Luma(255u8);
    let center = Point{x: 25.0, y: 0.0, z: -300.0};

    for x in range(0, x_size) {
        for y in range(0, y_size) {
            let start = Point{x: (x as f32 - x_size as f32 / 2.0) / div, y: (y as f32 - y_size as f32 / 2.0) / div, z: standoff as f32};
            let ray = Point{x: start.x - camera.x, y: start.y - camera.y, z: start.z - camera.z};
            let amp = 1.0 / ray.mag();
            let norm_ray = Point{x: ray.x * amp, y: ray.y * amp, z: ray.z * amp};
            match sphere_intersect(norm_ray, center, 20.0) {
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
