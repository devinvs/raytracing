use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use std::rc::Rc;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Rc<Box<Hittable>>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Box<Hittable>) {
        self.objects.push(Rc::from(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hittable in &self.objects {
            if hittable.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}
