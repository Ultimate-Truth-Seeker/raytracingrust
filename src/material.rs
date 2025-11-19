// material.rs
use raylib::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: f32,
    pub specular_strength: f32,
    pub shininess: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub ior: f32,

    pub emission: Color,         // emission color (in sRGB)
    pub emission_strength: f32,  // how bright it glows

    pub texture: Option<char>,
    pub anim_frames: u32,
    pub anim_fps: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse: Color::WHITE,
            albedo: 1.0,
            specular_strength: 0.0,
            shininess: 32.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 1.0,
            emission: Color::BLACK,
            emission_strength: 0.0,
            texture: None,
            anim_frames: 1,
            anim_fps: 0.0,
        }
    }
}


pub fn dirt() -> Material {
        Material {
            diffuse: Color::WHITE,
            albedo: 0.5,
            specular_strength: 0.0,
            shininess: 32.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 1.0,
            emission: Color::BLACK,
            emission_strength: 0.0,
            texture: Some('#'), 
            anim_frames: 1,
            anim_fps: 0.0,
        }
    }
    
pub fn grass() -> Material {
        Material {
            diffuse: Color::WHITE,
            albedo: 0.5,
            specular_strength: 0.0,
            shininess: 32.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 1.0,
            emission: Color::BLACK,
            emission_strength: 0.0,
            texture: Some('#'),  
            anim_frames: 1,
            anim_fps: 0.0,
        }
}

pub fn stone() -> Material {
    Material {
        diffuse: Color::WHITE,
        albedo: 0.5,
        specular_strength: 0.0,
        shininess: 32.0,
        reflectivity: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: Color::BLACK,
        emission_strength: 0.0,
        texture: Some('+'),
        anim_frames: 1,
        anim_fps: 0.0,
    }
}

pub fn obsidian() -> Material {
    Material {
        diffuse: Color::WHITE,
        albedo: 0.5,
        specular_strength: 0.6,
        shininess: 32.0,
        reflectivity: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: Color::BLACK,
        emission_strength: 0.0,
        texture: Some('-'),
        anim_frames: 1,
        anim_fps: 0.0,
    }
}

pub fn glass() -> Material {
    Material {
        diffuse: Color::WHITE,
        albedo: 1.0,
        specular_strength: 0.0,
        shininess: 32.0,
        reflectivity: 0.3,
        transparency: 0.65,
        ior: 1.4,
        emission: Color::BLACK,
        emission_strength: 0.0,
        texture: Some('-'),
        anim_frames: 1,
        anim_fps: 0.0,
    }
}

pub fn lamp() -> Material {
    Material { 
        diffuse: Color::WHITE, 
        albedo: 1.0, 
        specular_strength: 0.0, 
        shininess: 16.0, 
        reflectivity: 0.0, 
        transparency: 0.0, 
        ior: 1.0, 
        emission: Color::WHITE, 
        emission_strength: 3.0, 
        texture: Some('l'),
        anim_frames: 1,
        anim_fps: 0.0,
    }
}

pub fn portal() -> Material {
    Material { 
        diffuse: Color::WHITE, 
        albedo: 1.0, 
        specular_strength: 0.0, 
        shininess: 16.0, 
        reflectivity: 0.0, 
        transparency: 0.5, 
        ior: 1.0, 
        emission: Color::MAGENTA, 
        emission_strength: 0.5, 
        texture: Some('p'),
        anim_frames: 32,
        anim_fps: 24.0,
    }
}