use raylib::prelude::*;

// -------- utilidades de color --------
pub fn color_scale(c: Color, s: f32) -> Color {
    let r = (c.r as f32 * s).clamp(0.0, 255.0) as u8;
    let g = (c.g as f32 * s).clamp(0.0, 255.0) as u8;
    let b = (c.b as f32 * s).clamp(0.0, 255.0) as u8;
    Color::new(r, g, b, c.a)
}

pub fn color_mul(c: Color, f: f32) -> Color { color_scale(c, f) }

pub fn color_add(a: Color, b: Color) -> Color {
    Color::new(
        a.r.saturating_add(b.r),
        a.g.saturating_add(b.g),
        a.b.saturating_add(b.b),
        255,
    )
}

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let r = (a.r as f32 + (b.r as f32 - a.r as f32) * t) as u8;
    let g = (a.g as f32 + (b.g as f32 - a.g as f32) * t) as u8;
    let bch = (a.b as f32 + (b.b as f32 - a.b as f32) * t) as u8;
    Color::new(r, g, bch, 255)
}

pub fn srgb_to_linear(c: Color) -> (f32, f32, f32) {
    (c.r as f32 / 255.0,
    c.g as f32 / 255.0,
    c.b as f32 / 255.0)
}

pub fn linear_to_srgb(lr:f32, lg:f32, lb:f32) -> Color {
    let r = (lr.clamp(0.0, 1.0) * 255.0) as u8;
    let g = (lg.clamp(0.0, 1.0) * 255.0) as u8;
    let b = (lb.clamp(0.0, 1.0) * 255.0) as u8;
    Color::new(r, g, b, 255)
}