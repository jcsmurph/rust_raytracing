use crate::{vector::Vec3, material::Material, ray::Ray, hittable::{HitRecord, Hit}};


pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let oc = ray.origin() - self.center();
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius() * self.radius();
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;
            if t1 < t_max && t1 > t_min {
                let p = ray.point_at_parameter(t1);
                let n = (p - self.center()) / self.radius();
                Some((HitRecord::new(t1, p, n), self.material()))
            } else if t2 < t_max && t2 > t_min {
                let p = ray.point_at_parameter(t2);
                let n = (p - self.center()) / self.radius();
                Some((HitRecord::new(t2, p, n), self.material()))
            } else {
                None
            }
        } else {
            None
        }
    }
}
