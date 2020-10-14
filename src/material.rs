use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vector::Color;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray);
}
