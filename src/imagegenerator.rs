use std::fs;

use rand::Rng;
use rayon::prelude::{IntoParallelIterator, IndexedParallelIterator, ParallelIterator};

use crate::{camera::Camera, color, random_scene, vector::Vec3};

pub struct ImageGenerator {
    height: f32,
    width: f32,
    ray_per_pixel: f32,
}

impl ImageGenerator {
    pub fn new(height: f32, width: f32, ray_per_pixel: f32) -> ImageGenerator {
        ImageGenerator {
            height,
            width,
            ray_per_pixel,
        }
    }

    pub fn generate_image(&self) {
        let world = random_scene();
        let max_color = 255;

        println!("->Generating the image");

        let mut pic = format!("P3\n{} {}\n{}\n", self.width, self.height, max_color);
        let lookfrom = Vec3::new(16.0, 2.0, 4.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let dist_to_focus = (lookfrom - lookat).length();
        let aperture = 0.2;
        let camera = Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            15.0,
            f64::from(self.width) / f64::from(self.height),
            aperture,
            dist_to_focus,
        );

        let pixels = (0..self.height as i32)
            .into_par_iter()
            .rev()
            .map(|h| {
                (0..self.width as i32)
                    .into_par_iter()
                    .map(|w| {
                        let mut rng = rand::thread_rng();
                        let mut col = Vec3::new(0.0, 0.0, 0.0);
                        for _ in 0..self.ray_per_pixel as i32 {
                            let u = (f64::from(w) + rng.gen::<f64>()) / f64::from(self.width);
                            let v = (f64::from(h) + rng.gen::<f64>()) / f64::from(self.height);
                            let r = &camera.get_ray(u, v);
                            col = col + color(&r, &world, 0);
                        }
                        col = col / f64::from(self.ray_per_pixel);
                        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
                        let ir = (f64::from(max_color) * col.x) as usize;
                        let ig = (f64::from(max_color) * col.y) as usize;
                        let ib = (f64::from(max_color) * col.z) as usize;
                        format!("{} {} {}\n", ir, ig, ib)
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("");

        pic = format!("{}{}", &pic, pixels);

        println!(":: Writing the image");
        if fs::write("output.ppm", pic).is_err() {
            eprintln!("Could not generate the picture");
        };

        println!("-> Image has been created.")
    }
}
