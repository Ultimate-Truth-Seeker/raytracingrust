// material.rs
use raylib::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: f32,
    pub texture: Option<char>,   // <-- NEW: which texture to use, e.g. Some('#')
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse: Color::WHITE,
            albedo: 1.0,
            texture: None,
        }
    }
}

impl Material {
    fn dirt() -> Self {
        Self {
            diffuse: Color::WHITE,//new(200, 180, 140, 255),
            albedo: 0.5,
            texture: Some('#'),  // <-- uses assets/wall3.png per your map
        }
    }
    fn grass() -> Self {
        Self {
            diffuse: Color::WHITE,//new(200, 180, 140, 255),
            albedo: 0.5,
            texture: Some('#'),  // <-- uses assets/wall3.png per your map
        }
    }
}