use raylib::prelude::*;
use crate::material::Material;
use crate::ray_intersect::{Hit, RayIntersect};
use crate::object::obj::Obj;

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub material: Material,
}

impl Mesh {
    pub fn from_obj(
        obj: &Obj,
        material: Material,
        offset: Vector3,
        scale: f32,
    ) -> Self {
        let mut triangles = Vec::new();

        for (v0, v1, v2) in obj.get_triangles() {
            triangles.push(Triangle {
                v0: v0 * scale + offset,
                v1: v1 * scale + offset,
                v2: v2 * scale + offset,
            });
        }

        Mesh { triangles, material }
    }
}

impl RayIntersect for Mesh {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3, obj_id: usize) -> Hit {
        let mut closest = Hit::no_hit();

        for tri in &self.triangles {
            // Möller–Trumbore
            let v0v1 = tri.v1 - tri.v0;
            let v0v2 = tri.v2 - tri.v0;
            let pvec = rd.cross(v0v2);
            let det = v0v1.dot(pvec);

            if det.abs() < 1e-6 {
                continue; // ray parallel to triangle
            }

            let inv_det = 1.0 / det;
            let tvec = *ro - tri.v0;
            let u = tvec.dot(pvec) * inv_det;
            if u < 0.0 || u > 1.0 {
                continue;
            }

            let qvec = tvec.cross(v0v1);
            let v = rd.dot(qvec) * inv_det;
            if v < 0.0 || u + v > 1.0 {
                continue;
            }

            let t = v0v2.dot(qvec) * inv_det;
            if t <= 1e-4 {
                continue;
            }

            if t < closest.distance {
                let hit_point = *ro + *rd * t;

                // Face normal (flat shading)
                let mut n = v0v1.cross(v0v2).normalized();
                // Make sure it faces against the ray (same convention as your other objects)
                if rd.dot(n) > 0.0 {
                    n = -n;
                }

                // For now, no UVs → fake them from barycentrics (optional)
                let w = 1.0 - u - v;
                let uv = Vector2::new(u, v); // or (barycentric-based, but we don't use textures here)

                closest = Hit {
                    is_intersecting: true,
                    distance: t,
                    point: hit_point,
                    normal: n,
                    material: self.material,
                    uv,
                    obj_id,
                    tex_id: self.material.texture, // None for now
                };
            }
        }

        closest
    }
}