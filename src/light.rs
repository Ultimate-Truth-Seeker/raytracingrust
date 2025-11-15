use std::f32::consts::PI;

use raylib::prelude::*;

// -------- Luz puntual simple --------
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub position: Vector3,
    pub intensity: f32, // escala [0..∞), e.g. 1.0 = normal
    theta: f32,
}

impl PointLight {
    pub fn new(position: Vector3, intensity: f32) -> Self {
        let size = position.length();
        let mut theta = (position.x / size).acos();
        if position.y < 0.0 {
            theta = 2.0*PI - theta
        }
        PointLight { position, intensity, theta }
    }
    pub fn rotate(&mut self) {
        let rot_speed = PI / 75.0;
        self.theta = self.theta + rot_speed;
        let size = self.position.length();
        self.position.x = size * self.theta.cos();
        self.position.y = size * self.theta.sin();
    }
}