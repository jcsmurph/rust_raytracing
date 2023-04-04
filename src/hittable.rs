
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

pub struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait Hit: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)>;
}

pub struct HitBox {
    list: Vec<Box<dyn Hit>>,
}

impl HitBox {
    pub fn new(list: Vec<Box<dyn Hit>>) -> HitBox {
        HitBox { list }
    }
}

impl Hit for HitBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let mut closest_so_far = t_max;
        let mut res = None;
        for h in self.list.iter() {
            if let Some((hit_record, material)) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t();
                res = Some((
                    HitRecord::new(hit_record.t(), hit_record.p(), hit_record.normal()),
                    material,
                ))
            }
        }
        res
    }
}
