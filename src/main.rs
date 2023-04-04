mod camera;
mod gui;
mod hittable;
mod imagegenerator;
mod material;
mod ray;
mod sphere;
mod vector;

use gui::MyEguiApp;
use hittable::{Hit, HitBox};
use material::Material;
use ray::Ray;
use sphere::Sphere;

use vector::Vec3;

use rand::prelude::*;

fn color(r: &Ray, world: &dyn Hit, depth: u32) -> Vec3 {
    match world.hit(r, 0.001, std::f64::INFINITY) {
        Some((hit_record, material)) => {
            let n = hit_record.normal();
            let p = hit_record.p();
            let (scattered, attenuation, b) = material.scatter(r, n, p);
            if depth < 50 && b {
                attenuation * color(&scattered, world, depth + 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = r.direction().unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn random_scene() -> HitBox {
    let mut rng = rand::thread_rng();
    let mut list: Vec<Box<dyn Hit>> = vec![];
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            attenuation: Vec3::new(0.5, 0.5, 0.5),
        },
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian {
                            attenuation: Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ),
                        },
                    )));
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            attenuation: Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            fuzziness: 0.5 * rng.gen::<f64>(),
                        },
                    )));
                } else {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric { refraction: 1.5 },
                    )));
                }
            }
        }
    }
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric { refraction: 1.5 },
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            attenuation: Vec3::new(0.4, 0.2, 0.1),
        },
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            attenuation: Vec3::new(0.7, 0.6, 0.5),
            fuzziness: 0.0,
        },
    )));
    HitBox::new(list)
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    ).unwrap();
}
