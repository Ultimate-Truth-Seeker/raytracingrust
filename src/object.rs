use raylib::prelude::*;
use crate::{animated_quad::AnimatedQuad, cube::Cube, material::*, ray_intersect::{Hit, RayIntersect}, sphere::Sphere};

// -------- Objetos soportados --------
#[derive(Clone, Copy, Debug)]
pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
    AnimatedQuad(AnimatedQuad)
}

impl RayIntersect for Object {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3, obj_id: usize) -> Hit {
        match self {
            Object::Sphere(s) => s.ray_intersect(ro, rd, obj_id),
            Object::Cube(c)   => c.ray_intersect(ro, rd, obj_id),
            Object::AnimatedQuad(aq) => aq.ray_intersect(ro, rd, obj_id),
        }
    }
}

pub fn sample_objects() -> Vec<Object> {
    vec![
        //lights
        Object::Cube(Cube::new(-3.0, 1.0, 0.0, 'l')),
        Object::Cube(Cube::new(6.0, 1.0, -1.0, 'l')),

        Object::Cube(Cube::grass_block(0.0, -1.0, 0.0)),
        Object::Cube(Cube::new(0.0, 0.0, 0.0, 't')),
        Object::Cube(Cube::new(0.0, -2.0, 0.0, '+')),
        Object::Cube(Cube::new(0.0, -2.0, -1.0, '+')),

        //Object::Cube(Cube::new(1.0, -1.0, 1.0, '-')),
        Object::Cube(Cube::new(1.0, -1.0, 0.0, '-')),
        Object::Cube(Cube::new(1.0, -1.0, -1.0, '-')),
        //Object::Cube(Cube::new(1.0, -1.0, -2.0, '-')),

        Object::Cube(Cube::new(1.0, 0.0, -2.0, '-')),
        Object::Cube(Cube::new(1.0, 0.0, 1.0, '-')),
        Object::Cube(Cube::new(1.0, 1.0, -2.0, '-')),
        Object::Cube(Cube::new(1.0, 1.0, 1.0, '-')),
        Object::Cube(Cube::new(1.0, 2.0, -2.0, '-')),
        Object::Cube(Cube::new(1.0, 2.0, 1.0, '-')),

        Object::AnimatedQuad(AnimatedQuad::new(Vector3::new(1.5, 2.5, -0.5), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 1.0, 1.0, portal())),
        Object::AnimatedQuad(AnimatedQuad::new(Vector3::new(1.5, 1.5, -0.5), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 1.0, 1.0, portal())),
        Object::AnimatedQuad(AnimatedQuad::new(Vector3::new(1.5, 0.5, -0.5), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 1.0, 1.0, portal())),
        Object::AnimatedQuad(AnimatedQuad::new(Vector3::new(1.5, 2.5, 0.5), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 1.0, 1.0, portal())),
        Object::AnimatedQuad(AnimatedQuad::new(Vector3::new(1.5, 1.5, 0.5), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 1.0, 1.0, portal())),
        Object::AnimatedQuad(AnimatedQuad::new(Vector3::new(1.5, 0.5, 0.5), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 1.0, 1.0, portal())),
       

        //Object::Cube(Cube::new(1.0, 3.0, 1.0, '-')),
        Object::Cube(Cube::new(1.0, 3.0, 0.0, '-')),
        Object::Cube(Cube::new(1.0, 3.0, -1.0, '-')),
        //Object::Cube(Cube::new(1.0, 3.0, -2.0, '-')),


    ]
}