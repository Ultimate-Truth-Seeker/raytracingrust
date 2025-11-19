// cube.rs
use raylib::prelude::*;
use crate::material::{Material, *};
use crate::ray_intersect::{Hit, RayIntersect};

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub material: Material,
    pub face_textures: [Option<char>; 6],
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3, obj_id: usize) -> Hit {
        // Safe inverses
        let invx = if rd.x.abs() < 1e-8 {
            1.0 / (if rd.x.is_sign_negative() { -1.0e-8 } else { 1.0e-8 })
        } else { 1.0 / rd.x };

        let invy = if rd.y.abs() < 1e-8 {
            1.0 / (if rd.y.is_sign_negative() { -1.0e-8 } else { 1.0e-8 })
        } else { 1.0 / rd.y };

        let invz = if rd.z.abs() < 1e-8 {
            1.0 / (if rd.z.is_sign_negative() { -1.0e-8 } else { 1.0e-8 })
        } else { 1.0 / rd.z };

        // X slabs
        let mut tx1 = (self.min.x - ro.x) * invx;
        let mut tx2 = (self.max.x - ro.x) * invx;
        let (fx1, fx2) = if rd.x >= 0.0 { (-1, 1) } else { (1, -1) };
        if tx1 > tx2 { std::mem::swap(&mut tx1, &mut tx2); }

        // Y slabs
        let mut ty1 = (self.min.y - ro.y) * invy;
        let mut ty2 = (self.max.y - ro.y) * invy;
        let (fy1, fy2) = if rd.y >= 0.0 { (-2, 2) } else { (2, -2) };
        if ty1 > ty2 { std::mem::swap(&mut ty1, &mut ty2); }

        // Z slabs
        let mut tz1 = (self.min.z - ro.z) * invz;
        let mut tz2 = (self.max.z - ro.z) * invz;
        let (fz1, fz2) = if rd.z >= 0.0 { (-3, 3) } else { (3, -3) };
        if tz1 > tz2 { std::mem::swap(&mut tz1, &mut tz2); }

        // Enter / exit
        let mut t_enter = tx1;
        let mut face_enter = fx1;
        if ty1 > t_enter { t_enter = ty1; face_enter = fy1; }
        if tz1 > t_enter { t_enter = tz1; face_enter = fz1; }

        let mut t_exit = tx2;
        let mut face_exit = fx2;
        if ty2 < t_exit { t_exit = ty2; face_exit = fy2; }
        if tz2 < t_exit { t_exit = tz2; face_exit = fz2; }

        if t_enter > t_exit || t_exit < 1e-5 {
            return Hit::no_hit();
        }

        let (t, face) = if t_enter > 1e-5 {
            (t_enter, face_enter)
        } else {
            (t_exit, -face_exit)
        };

        let point = *ro + *rd * t;

        let normal = match face {
            -1 => Vector3::new(-1.0, 0.0, 0.0),
            1  => Vector3::new( 1.0, 0.0, 0.0),
            -2 => Vector3::new(0.0, -1.0, 0.0),
            2  => Vector3::new(0.0,  1.0, 0.0),
            -3 => Vector3::new(0.0, 0.0, -1.0),
            3  => Vector3::new(0.0, 0.0,  1.0),
            _  => Vector3::new(0.0, 0.0,  1.0),
        };

        // --- UVs per face: map face rectangle to [0,1]x[0,1] ---
        let size = self.max - self.min;
        let mut u = 0.0;
        let mut v = 0.0;

        match face {
            // -X / +X: use (z,y)
            -1 => {
                u = (point.z - self.min.z) / size.z;
                v = (point.y - self.min.y) / size.y;
            },
            1 => {
                u = 1.0 - (point.z - self.min.z) / size.z; // flip to keep orientation
                v = (point.y - self.min.y) / size.y;
            },

            // -Y / +Y: use (x,z)
            -2 => {
                u = (point.x - self.min.x) / size.x;
                v = (point.z - self.min.z) / size.z;
            },
            2 => {
                u = (point.x - self.min.x) / size.x;
                v = 1.0 - (point.z - self.min.z) / size.z;
            },

            // -Z / +Z: use (x,y)
            -3 => {
                u = (point.x - self.min.x) / size.x;
                v = (point.y - self.min.y) / size.y;
            },
            3 => {
                u = 1.0 - (point.x - self.min.x) / size.x;
                v = (point.y - self.min.y) / size.y;
            },
            _ => {}
        }

        // Clamp to [0,1] in case of tiny numeric overshoot
        u = u.clamp(0.0, 1.0);
        v = v.clamp(0.0, 1.0);

        Hit {
            is_intersecting: true,
            distance: t,
            point,
            normal,
            material: self.material,
            uv: Vector2::new(u, v),
            obj_id,
            tex_id: self.tex_for_face(face),
        }
    }
}

impl Cube {
    pub fn new(x:f32, y:f32, z:f32, tex: char) -> Self {
        Cube {
            min: Vector3::new(x, y, z),
            max: Vector3::new( x + 1.0,  y + 1.0, z + 1.0),
            material: match tex {
                '+' => stone(),
                '-' => obsidian(),
                '#' => dirt(),
                't' => glass(),
                'l' => lamp(),
                _ => dirt(),
            },
            face_textures: [
                    Some(tex),Some(tex),Some(tex),Some(tex),Some(tex),Some(tex),
                ],
        }
    }
    pub fn grass_block(x:f32, y:f32, z:f32) -> Self {
        Cube {
            min: Vector3::new(x, y, z),
            max: Vector3::new( x + 1.0,  y + 1.0, z + 1.0),
            material: grass(),
            face_textures: [
                    Some('|'),Some('|'),Some('g'),Some('#'),Some('|'),Some('|'),
                ],
        }
    }

    fn tex_for_face(&self, face: i32) -> Option<char> {
        match face {
            1  => self.face_textures[0], // +X
            -1 => self.face_textures[1], // -X
            2  => self.face_textures[2], // +Y
            -2 => self.face_textures[3], // -Y
            3  => self.face_textures[4], // +Z
            -3 => self.face_textures[5], // -Z
            _  => None,
        }
    }
}