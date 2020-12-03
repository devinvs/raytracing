use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vector::{Color, Vec3};
use rand;
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {
            albedo: a
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        return true;
    }
}



pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32
}

impl Metal {
    pub fn new(a: Color, fuzz: f32) -> Metal {
        if fuzz > 1.0 {
            let fuzz = 1.0;
        }

        Metal {
            albedo: a,
            fuzz
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = ray_in.direction
            .unit_vector()
            .reflect(&rec.normal);

        *scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_unit_vector());
        *attenuation = self.albedo;

        scattered.direction.dot(&rec.normal) > 0.0
    }
}


pub struct Dielectric {
    ir: f32
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric {
            ir
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0*r0;
        r0 + (1.0-r0)*(1.0-cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut rng = rand::thread_rng();
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = {
            let a = (-unit_direction).dot(&rec.normal);
            let b = 1.0;

            if a > b { a } else { b }
        };
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }
}
