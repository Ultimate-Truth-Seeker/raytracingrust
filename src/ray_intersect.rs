// ray_intersect.rs
use raylib::prelude::*;

use crate::material::Material;

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub is_intersecting: bool,
    pub distance: f32,
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Material,
    pub uv: Vector2,
    pub obj_id: usize,
    pub tex_id: Option<char>
}

impl Hit {
    pub fn no_hit() -> Self {
        Self {
            is_intersecting: false,
            distance: f32::INFINITY,
            point: Vector3::zero(),
            normal: Vector3::zero(),
            material: Material::default(),
            uv: Vector2::zero(),
            obj_id: 0,
            tex_id: None,
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_dir: &Vector3, obj_id: usize) -> Hit;
}