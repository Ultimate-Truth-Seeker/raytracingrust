use raylib::prelude::*;
// Reflect incident vector `i` around normal `n`
pub fn reflect(i: Vector3, n: Vector3) -> Vector3 {
    i - n * (2.0 * i.dot(n))
}

// Refract incident vector `i` given normal `n` and indices `eta_i` (from) and `eta_t` (to)
pub fn refract(i: Vector3, n: Vector3, eta_i: f32, eta_t: f32) -> Option<Vector3> {
    let mut cosi = (-i.dot(n)).clamp(-1.0, 1.0);
    let mut etai = eta_i;
    let mut etat = eta_t;
    let mut n_ref = n;

    // If we’re inside the object, invert normal and swap indices
    if cosi < 0.0 {
        cosi = -cosi;
        std::mem::swap(&mut etai, &mut etat);
        n_ref = Vector3::new(-n.x, -n.y, -n.z);
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        None // total internal reflection
    } else {
        Some(i * eta + n_ref * (eta * cosi - k.sqrt()))
    }
}

pub fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
    // cos_theta in [0,1], f0 in [0,1]
    f0 + (1.0 - f0) * (1.0 - cos_theta).powf(5.0)
}

pub fn f0_from_ior(ior: f32) -> f32 {
    // for dielectrics
    let r0 = (ior - 1.0) / (ior + 1.0);
    r0 * r0
}