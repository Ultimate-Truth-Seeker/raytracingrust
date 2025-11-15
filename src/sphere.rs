// sphere.rs
use raylib::prelude::*;
use crate::material::Material;
use crate::ray_intersect::{Hit, RayIntersect};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3) -> Hit {
        // Ray-sphere: (ro + t rd - c)^2 = r^2
        let oc = *ro - self.center;
        let a = rd.dot(*rd);
        let b = 2.0 * oc.dot(*rd);
        let c = oc.dot(oc) - self.radius * self.radius;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return Hit::no_hit();
        }

        let sqrt_disc = disc.sqrt();
        let mut t = (-b - sqrt_disc) / (2.0 * a);
        if t <= 1e-4 {
            t = (-b + sqrt_disc) / (2.0 * a);
            if t <= 1e-4 {
                return Hit::no_hit();
            }
        }

        let point = *ro + *rd * t;
        let normal = (point - self.center).normalized();

        Hit {
            is_intersecting: true,
            distance: t,
            point,
            normal,
            material: self.material,
        }
    }
}