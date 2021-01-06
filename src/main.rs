use surt::camera::Camera;
use surt::color::ColorRGB;
use surt::materials::Material;
use surt::ray::Ray;
use surt::sphere::Sphere;
use surt::utils::clamp;
use surt::vector::Vector3d;
use surt::world::World;

use pixel_canvas::{input::MouseState, Canvas, Color};
use rand::Rng;
use std::f64::consts::PI;
// use std::fs;

fn ray_color(ray: &Ray, world: &World, depth: i32) -> ColorRGB {
    if depth <= 0 {
        return ColorRGB::BLACK;
    }

    match world.hit(ray, 0.001, f64::MAX) {
        Some(rec) => match rec.material.scatter(ray, &rec) {
            Some((ray, att)) => {
                return ray_color(&ray, world, depth - 1) * att;
            }
            None => {
                return ColorRGB::BLACK;
            }
        },
        None => {
            let unit_dir = ray.dir.unit_vec();
            let t = 0.5 * (unit_dir.y + 1.0);
            let grad_beg = ColorRGB::WHITE;
            let grad_end = ColorRGB::new(0.5, 0.7, 1.0);
            return grad_beg * (1.0 - t) + grad_end * t;
        }
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 720;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 1;
    let max_depth: i32 = 50;

    let mut world = World::default();
    world.add(Box::new(Sphere {
        center: Vector3d::new(0.0, 0.0, -0.3),
        radius: 0.1,
        material: Material::Dielectric { ir: 1.7 },
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(0.0, 0.0, -0.3),
        radius: 0.03,
        material: Material::Dielectric { ir: -0.4 },
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(0.5, 0.9, -1.0),
        radius: 0.2,
        material: Material::Lambertian {
            albedo: ColorRGB::new(0.8, 0.8, 0.0),
        },
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(1.0, -0.3, -1.0),
        radius: 0.4,
        material: Material::Metal {
            albedo: ColorRGB::new(0.8, 0.2, 0.1),
            fuzz: 0.1,
        },
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(-1.0, -0.1, -1.0),
        radius: 0.3,
        material: Material::Lambertian {
            albedo: ColorRGB::new(0.1, 0.2, 0.4),
        },
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(-0.2, 0.2, -0.6),
        radius: 0.2,
        material: Material::Metal {
            albedo: ColorRGB::new(0.4, 0.4, 0.4),
            fuzz: 0.0,
        },
    }));

    world.add(Box::new(Sphere {
        center: Vector3d::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::Lambertian {
            albedo: ColorRGB::new(0.8, 0.8, 0.8),
        },
    }));
 
    let mut camera = Camera::new(
        Vector3d::new(0.0, 0.0, 0.0),
        Vector3d::new(0.0, 0.0, 0.0),
        60.0,
        aspect_ratio,
    );

    let canvas = Canvas::new(image_width as usize, image_height as usize)
        .title("SURT")
        .show_ms(true)
        .state(MouseState::new())
        .input(MouseState::handle_input);
    // The canvas will render for you at up to 60fps.
    canvas.render(move |mouse, image| {
        // Modify the `image` based on your state.
        
        let cam_angle_x = -((mouse.x as f64 / image_width  as f64) * 2.0 * PI).sin();
        let cam_angle_y = -((mouse.y as f64 / image_height as f64) * 1.0 * PI).cos() * 2.0;
        // let cam_angle_z = -1.0;
        let cam_angle_z = ((mouse.x as f64 / image_width  as f64) * 2.0 * PI).cos();
        println!("x: {}, y: {}", cam_angle_x, cam_angle_y);

        camera.update_angle(Vector3d::new(cam_angle_x, cam_angle_y, cam_angle_z));
        

        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let mut pixel_color = ColorRGB::BLACK;
                for _ in 0..samples_per_pixel {
                    let mut rng = rand::thread_rng();
                    let u = (x as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                    let v = (y as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, max_depth);
                }
                let sample_scale = 1.0 / samples_per_pixel as f64;
                pixel_color = pixel_color * sample_scale;

                *pixel = Color {
                    r: (256.0 * clamp(pixel_color.r.sqrt(), 0.0, 0.999)) as u8,
                    g: (256.0 * clamp(pixel_color.g.sqrt(), 0.0, 0.999)) as u8,
                    b: (256.0 * clamp(pixel_color.b.sqrt(), 0.0, 0.999)) as u8,
                }
            }
        }
    });

    // let mut scene: Vec<ColorRGB> = Vec::with_capacity((image_width * image_height) as usize);
    // for j in (0..image_height).rev() {
    //     for i in 0..image_width {
    //         let mut pixel_color = ColorRGB::BLACK;
    //         for _ in 0..samples_per_pixel {
    //             let mut rng = rand::thread_rng();
    //             let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
    //             let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
    //             let ray = camera.get_ray(u, v);
    //             pixel_color += ray_color(&ray, &world, max_depth);
    //         }
    //         let sample_scale = 1.0 / samples_per_pixel as f64;

    //         scene.push(pixel_color * sample_scale);
    //     }
    // }

    // let mut image: String = String::with_capacity((image_width * image_height * 12 + 20) as usize);
    // let ppm_header = format!("P3\n{} {} \n255\n", image_width, image_height);
    // image.push_str(&ppm_header);
    // for pixel in &scene {
    //     image.push_str(&pixel.to_ppm());
    // }

    // fs::write("./image.ppm", image).expect("Failed to write to file");
}
