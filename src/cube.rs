// cube.rs
use raylib::prelude::*;
use crate::material::Material;
use crate::ray_intersect::{Hit, RayIntersect};

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub material: Material,
}

impl Cube {
    #[inline]
    fn axis_slab(ro: f32, rd: f32, minv: f32, maxv: f32) -> (f32, f32, i32, i32) {
        // returns (t_enter, t_exit, face_enter_id, face_exit_id)
        // face ids: 0/-0 = ±X, 1/-1 = ±Y, 2/-2 = ±Z (we’ll map to normals later)
        const BIG: f32 = 1.0e30;
        let inv = if rd.abs() < 1e-8 { 1.0 / (if rd.is_sign_negative() { -1.0e-8 } else { 1.0e-8 }) } else { 1.0 / rd };

        let mut t1 = (minv - ro) * inv;
        let mut t2 = (maxv - ro) * inv;

        let (mut f1, mut f2) = (0, 0);
        if rd >= 0.0 {
            // entering via min-plane, exiting via max-plane
            f1 = if minv == minv { // sanity
                // min-plane
                match () { _ => 0 } // placeholder, set by caller per axis
            } else { 0 };
            f2 = 0; // set by caller
        } else {
            // if rd < 0, t1 and t2 swap semantic roles
            std::mem::swap(&mut t1, &mut t2);
        }
        (t1, t2, f1, f2)
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3) -> Hit {
        // For each axis, compute t_enter/t_exit and face ids explicitly.
        let invx = if rd.x.abs() < 1e-8 { 1.0 / (if rd.x.is_sign_negative() { -1.0e-8 } else { 1.0e-8 }) } else { 1.0 / rd.x };
        let invy = if rd.y.abs() < 1e-8 { 1.0 / (if rd.y.is_sign_negative() { -1.0e-8 } else { 1.0e-8 }) } else { 1.0 / rd.y };
        let invz = if rd.z.abs() < 1e-8 { 1.0 / (if rd.z.is_sign_negative() { -1.0e-8 } else { 1.0e-8 }) } else { 1.0 / rd.z };

        // X
        let mut tx1 = (self.min.x - ro.x) * invx;
        let mut tx2 = (self.max.x - ro.x) * invx;
        let (fx1, fx2) = if rd.x >= 0.0 { (-1, 1) } else { (1, -1) }; // -X enter, +X exit if rd.x>=0
        if tx1 > tx2 { std::mem::swap(&mut tx1, &mut tx2); }

        // Y
        let mut ty1 = (self.min.y - ro.y) * invy;
        let mut ty2 = (self.max.y - ro.y) * invy;
        let (fy1, fy2) = if rd.y >= 0.0 { (-2, 2) } else { (2, -2) };
        if ty1 > ty2 { std::mem::swap(&mut ty1, &mut ty2); }

        // Z
        let mut tz1 = (self.min.z - ro.z) * invz;
        let mut tz2 = (self.max.z - ro.z) * invz;
        let (fz1, fz2) = if rd.z >= 0.0 { (-3, 3) } else { (3, -3) };
        if tz1 > tz2 { std::mem::swap(&mut tz1, &mut tz2); }

        // Enter is the max of axis enters; Exit is the min of axis exits.
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

        // If outside, hit at enter; if starting inside, hit at exit (and flip normal)
        let (t, face) = if t_enter > 1e-5 {
            (t_enter, face_enter)
        } else {
            (t_exit, -face_exit) // exiting: normal points inward face -> flip
        };

        let point = *ro + *rd * t;

        let normal = match face {
            -1 => Vector3::new(-1.0, 0.0, 0.0),
            1  => Vector3::new( 1.0, 0.0, 0.0),
            -2 => Vector3::new(0.0, -1.0, 0.0),
            2  => Vector3::new(0.0,  1.0, 0.0),
            -3 => Vector3::new(0.0, 0.0, -1.0),
            3  => Vector3::new(0.0, 0.0,  1.0),
            _  => Vector3::new(0.0, 0.0, 1.0),
        };

        Hit {
            is_intersecting: true,
            distance: t,
            point,
            normal,
            material: self.material,
        }
    }
}