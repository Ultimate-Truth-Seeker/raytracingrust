// animated_quad.rs
use raylib::prelude::*;
use crate::material::Material;
use crate::ray_intersect::{Hit, RayIntersect};

#[derive(Clone, Copy, Debug)]
pub struct AnimatedQuad {
    pub center: Vector3,
    pub normal: Vector3,
    pub up: Vector3,     // “vertical” axis on the quad
    pub width: f32,
    pub height: f32,
    pub material: Material,
}

impl AnimatedQuad {
    pub fn new(
        center: Vector3,
        normal: Vector3,
        up: Vector3,
        width: f32,
        height: f32,
        material: Material,
    ) -> Self {
        let n = normal.normalized();
        let mut u = up;
        if u.length() == 0.0 {
            u = Vector3::new(0.0, 1.0, 0.0);
        }
        // make up orthogonal to normal
        u = (u - n * u.dot(n)).normalized();

        Self {
            center,
            normal: n,
            up: u,
            width,
            height,
            material,
        }
    }
}

impl RayIntersect for AnimatedQuad {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3, obj_id: usize) -> Hit {
        let n = self.normal;
        let denom = n.dot(*rd);
        // Ray nearly parallel to plane
        if denom.abs() < 1e-6 {
            return Hit::no_hit();
        }

        let t = (self.center - *ro).dot(n) / denom;
        if t <= 1e-4 {
            return Hit::no_hit();
        }

        let p = *ro + *rd * t;

        // Build tangent/right axis
        let right = n.cross(self.up).normalized();
        let up = right.cross(n).normalized();

        // Local coordinates relative to center
        let rel = p - self.center;

        // Project onto quad axes (right, up)
        let x = rel.dot(right);
        let y = rel.dot(up);

        let hw = self.width * 0.5;
        let hh = self.height * 0.5;

        // Outside the square
        if x < -hw || x > hw || y < -hh || y > hh {
            return Hit::no_hit();
        }

        // Map to UV in [0,1]
        let u = (x / hw) * 0.5 + 0.5;
        let v = (y / hh) * 0.5 + 0.5;

        // For shading, we want normal to face against the ray
        let mut shaded_normal = n;
        if rd.dot(shaded_normal) > 0.0 {
            shaded_normal = -shaded_normal;
        }

        Hit {
            is_intersecting: true,
            distance: t,
            point: p,
            normal: shaded_normal,
            material: self.material,
            uv: Vector2::new(u, v),
            obj_id,
            tex_id: self.material.texture,  // or per-face if you prefer
        }
    }
}