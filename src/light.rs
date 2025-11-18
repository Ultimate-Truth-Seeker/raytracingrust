use std::f32::consts::PI;

use raylib::prelude::*;

use crate::object::Object;

// -------- Luz puntual simple --------
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub position: Vector3,
    pub intensity: f32, // escala [0..∞), e.g. 1.0 = normal
    pub emitter_index: Option<usize>,
    theta: f32,
}

impl PointLight {
    pub fn new(position: Vector3, intensity: f32, emitter_index: Option<usize>) -> Self {
        let size = position.length();
        let mut theta = (position.x / size).acos();
        if position.y < 0.0 {
            theta = 2.0*PI - theta
        }
        PointLight { position, intensity, emitter_index, theta }
    }
    pub fn rotate(&mut self) {
        let rot_speed = PI / 75.0;
        self.theta = self.theta + rot_speed;
        let size = self.position.length();
        self.position.x = size * self.theta.cos();
        self.position.y = size * self.theta.sin();
    }
}

pub fn build_lights_from_objects(objects: &[Object]) -> Vec<PointLight> {
    let mut lights = Vec::new();
    
    // Add a point light for each emissive object
    for (i, obj) in objects.iter().enumerate() {
        match obj {
            Object::Sphere(s) => {
                if s.material.emission_strength > 0.0 {
                    lights.push(PointLight::new(
                        s.center,
                        s.material.emission_strength,
                        Some(i),
                    ));
                }
            }
            Object::Cube(c) => {
                if c.material.emission_strength > 0.0 {
                    let center = (c.min + c.max) * 0.5;
                    lights.push(PointLight::new(
                        center,
                        c.material.emission_strength,
                        Some(i),
                    ));
                }
            }
        }
    }

    lights
}