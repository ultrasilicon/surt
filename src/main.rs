use surt::color::ColorRGB;
use surt::ray::Ray;
use surt::vector::Vector3D;

use std::fs;

fn hit_sphere(center: Vector3D, radius: f64, ray: &Ray) -> bool {
    let oc: Vector3D = ray.orig - center;
    let a = Vector3D::dot(ray.dir, ray.dir);
    let b = Vector3D::dot(oc, ray.dir) * 2.0;
    let c = Vector3D::dot(oc, oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;
    return discriminant > 0.0;
}

fn ray_color(ray: &Ray) -> ColorRGB {
    if hit_sphere(Vector3D{x: 0.0, y: 0.0, z: -1.0}, 0.5, ray) {
        return ColorRGB{r: 0.0, g: 0.0, b: 0.0}
    }
    let unit_dir = ray.dir.unit_vec();
    let t = 0.5 * (unit_dir.y + 1.0);
    let grad_beg = ColorRGB {
        r: 0.4,
        g: 0.4,
        b: 0.4,
    };
    let grad_end = ColorRGB {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    return grad_beg * (1.0 - t) + grad_end * t;
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 1080;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Vector3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vector3D {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vector3D {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };

    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vector3D {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    let mut image: String = format!("P3\n{} {} \n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let ray = Ray {
                orig: origin,
                dir: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            let color = ray_color(&ray);

            image.push_str(&color.to_ppm());
        }
    }

    fs::write("./image.ppm", image).expect("Failed to write to file");
}
