use surt::camera::Camera;
use surt::color::ColorRGB;
use surt::ray::Ray;
use surt::sphere::Sphere;
use surt::vector::Vector3d;
use surt::world::World;

use std::fs;
use rand::Rng;

fn ray_color(ray: &Ray, world: &World) -> ColorRGB {
    let result = world.hit(ray, 0.0, f64::MAX);
    match result {
        Some(rec) => {
            return (ColorRGB::WHITE + rec.normal) * 0.5;
        }
        None => {
            let unit_dir = ray.dir.unit_vec();
            let t = 0.5 * (unit_dir.y + 1.0);
            let grad_beg = ColorRGB::new(0.4, 0.4, 0.4);
            let grad_end = ColorRGB::BLACK;
            return grad_beg * (1.0 - t) + grad_end * t;
        }
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 1920;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;

    let mut world = World::default();
    world.add(Box::new(Sphere {
        center: Vector3d::new(0.0, 0.0, -1.0),
        radius: 0.1,
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(0.5, 0.9, -1.0),
        radius: 0.2,
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(1.0, -0.3, -1.0),
        radius: 0.3,
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(-1.0, -0.1, -1.0),
        radius: 0.3,
    }));


    world.add(Box::new(Sphere {
        center: Vector3d::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));


    let camera = Camera::new(aspect_ratio, 2.0, 1.0);

    let mut image: String = format!("P3\n{} {} \n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = ColorRGB::BLACK;
            for _ in 0..samples_per_pixel {
                let mut rng = rand::thread_rng();

                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            let sample_scale = 1.0 / samples_per_pixel as f64;
            
            image.push_str(&(pixel_color*sample_scale).to_ppm());
        }
    }

    fs::write("./image.ppm", image).expect("Failed to write to file");
}
