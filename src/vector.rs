use auto_ops::*;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Vec3([f32; 3]);
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            0: [x, y, z]
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self::new(
            rng.gen(),
            rng.gen(),
            rng.gen()
        )
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::random_range(-1.0, 1.0);

        while p.length_squared() >= 1.0 {
            p = Vec3::random_range(-1.0, 1.0);
        }

        p
    }

    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();

        let a = rng.gen_range(0.0, 2.0*PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = ((1.0-z*z) as f32).sqrt();

        Vec3::new(r*a.cos(), r*a.sin(), z)
    }

    pub fn x(&self) -> f32 { self.0[0] }
    pub fn y(&self) -> f32 { self.0[1] }
    pub fn z(&self) -> f32 { self.0[2] }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x()*self.x() + self.y()*self.y() + self.z()*self.z()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            0: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x()
            ]
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: &Vec3, etai_over_eta: f32) -> Vec3 {
        let cos_theta = (-self).dot(normal);
        let r_out_perp = etai_over_eta * (self + cos_theta*normal);
        let r_out_parallel = -(1.0-r_out_perp.length_squared()).abs().sqrt() * normal;

        r_out_perp + r_out_parallel
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

// Operations between two vectors

impl_op_ex! (- |a: &Vec3| -> Vec3 {
    Vec3 {
        0: [
            -a.x(),
            -a.y(),
            -a.z()
        ]
    }
});


impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { 
    Vec3{
        0: [
            a.x() + b.x(),
            a.y() + b.y(),
            a.z() + b.z()
        ]
    }
});

impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| {
    *a = Vec3 {
        0: [
            a.x() + b.x(),
            a.y() + b.y(),
            a.z() + b.z()
        ]
    };
});

impl_op_ex! (- |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        0: [
            a.x() - b.x(),
            a.y() - b.y(),
            a.z() - b.z()
        ]
    }
});

impl_op_ex! (-= |a: &mut Vec3, b: &Vec3| {
    *a = Vec3 {
        0: [
            a.x() - b.x(),
            a.y() - b.y(),
            a.z() - b.z()
        ]
    }
});

impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        0: [
            a.x() * b.x(),
            a.y() * b.y(),
            a.z() * b.z()
        ]
    }
});

impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| {
    *a = Vec3 {
        0: [
            a.x() * b.x(),
            a.y() * b.y(),
            a.z() * b.z()
        ]
    };
});

impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        0: [
            a.x() / b.x(),
            a.y() / b.y(),
            a.z() / b.z()
        ]
    }
});

impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| {
    *a = Vec3 {
        0: [
            a.x() / b.x(),
            a.y() / b.y(),
            a.z() / b.z()
        ]
    };
});


// Operations between a vector and a float

impl_op_ex!(+ |a: &Vec3, b: &f32| -> Vec3 {
    Vec3 {
        0: [
            a.x() + b,
            a.y() + b,
            a.z() + b
        ]
    }
});

impl_op_ex!(+= |a: &mut Vec3, b: &f32| {
    *a = Vec3 {
        0: [
            a.x() + b,
            a.y() + b,
            a.z() + b
        ]
    };
});

impl_op_ex!(-= |a: &mut Vec3, b: &f32| {
    *a = Vec3 {
        0: [
            a.x() - b,
            a.y() - b,
            a.z() - b
        ]
    };
});

impl_op_ex_commutative!(* |a: &Vec3, b: &f32| -> Vec3 {
    Vec3 {
        0: [
            a.x() * b,
            a.y() * b,
            a.z() * b
        ]
    }
});

impl_op_ex!(*= |a: &mut Vec3, b: &f32| {
    *a = Vec3 {
        0: [
            a.x() * b,
            a.y() * b,
            a.z() * b
        ]
    };
});

impl_op_ex!(/ |a: &Vec3, b: &f32| -> Vec3 {
    a * (1.0/b)
});

impl_op_ex!(/= |a: &mut Vec3, b: &f32| {
    let c = 1.0/b;
    *a = Vec3 {
        0: [
            a.x() / c,
            a.y() / c,
            a.z() / c
        ]
    }
});
