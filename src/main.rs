// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod sphere;
mod material;
mod cube;
mod camera;
mod textures;
mod light;

use framebuffer::Framebuffer;
use ray_intersect::{RayIntersect, Hit};
use sphere::Sphere;
use cube::Cube;
use material::Material;
use camera::Camera;
use textures::TextureManager;
use light::PointLight;

// -------- Objetos soportados --------
#[derive(Clone, Copy, Debug)]
pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
}

impl RayIntersect for Object {
    fn ray_intersect(&self, ro: &Vector3, rd: &Vector3) -> Hit {
        match self {
            Object::Sphere(s) => s.ray_intersect(ro, rd),
            Object::Cube(c)   => c.ray_intersect(ro, rd),
        }
    }
}

// -------- utilidades de color --------
fn color_scale(c: Color, s: f32) -> Color {
    let r = (c.r as f32 * s).clamp(0.0, 255.0) as u8;
    let g = (c.g as f32 * s).clamp(0.0, 255.0) as u8;
    let b = (c.b as f32 * s).clamp(0.0, 255.0) as u8;
    Color::new(r, g, b, c.a)
}

fn color_mul(c: Color, f: f32) -> Color { color_scale(c, f) }

fn color_add(a: Color, b: Color) -> Color {
    Color::new(
        a.r.saturating_add(b.r),
        a.g.saturating_add(b.g),
        a.b.saturating_add(b.b),
        255,
    )
}

// -------- UV helpers (box mapping from hit point & normal) --------
fn box_uv_from_world(p: Vector3, n: Vector3, tile: f32) -> (f32, f32) {
    let ax = n.x.abs();
    let ay = n.y.abs();
    let az = n.z.abs();

    let (u, v) = if ax >= ay && ax >= az {
        // X-dominant face: use (Z, Y). Flip U depending on face side so seams align.
        let u = if n.x > 0.0 { -p.z } else {  p.z };
        let v = p.y;
        (u * tile, v * tile)
    } else if ay >= ax && ay >= az {
        // Y-dominant face: use (X, Z)
        let u = p.x;
        let v = if n.y > 0.0 { -p.z } else {  p.z };
        (u * tile, v * tile)
    } else {
        // Z-dominant face: use (X, Y)
        let u = if n.z > 0.0 {  p.x } else { -p.x };
        let v = p.y;
        (u * tile, v * tile)
    };

    // Tile to [0,1)
    let uu = (u - u.floor()).fract();
    let vv = (v - v.floor()).fract();
    (uu, vv)
}

// -------- trazado con Lambert + sombra --------
pub fn cast_ray(
    ro: &Vector3,
    rd: &Vector3,
    objects: &[Object],
    lights: &[PointLight],
    texmgr: &TextureManager,            // <-- NEW
) -> Color {
    // Buscar el hit más cercano
    let mut closest = Hit::no_hit();
    for obj in objects {
        let h = obj.ray_intersect(ro, rd);
        if h.is_intersecting && h.distance < closest.distance {
            closest = h;
        }
    }
    if !closest.is_intersecting {
        return Color::new(4, 12, 36, 255); // fondo
    }

    // Base color: either sample texture or use diffuse
    let mut base = closest.material.diffuse;
    let tex_id = closest.tex_id.or(closest.material.texture);
    if let Some(ch) = tex_id {
        // try smaller tiling for less aliasing
        let (u, v) = (closest.uv.x, closest.uv.y);//box_uv_from_world(closest.point, closest.normal, 0.25);
        base = texmgr.sample_uv_bilinear(ch, u, v);
    }

    // Iluminación difusa (Lambert) + sombras duras
    let ambient = 0.08;
    let mut out_color = color_mul(base, ambient);

    for light in lights {
        let to_light = light.position - closest.point;
        let light_dist = to_light.length();
        let l_dir = to_light / light_dist;

        // Shadow ray
        //let eps = 1e-3;
        //let shadow_origin = closest.point + closest.normal * eps;

        // Before casting the shadow ray
        let ndotl = closest.normal.dot(l_dir).clamp(-1.0, 1.0);

        // Angle-aware bias: bigger bias for grazing angles
        let bias = 5e-3 + 5e-3 * (1.0 - ndotl.abs());

        // Two-sided: push along +N if light is above the surface, else along -N
        let shadow_origin = closest.point + closest.normal * if ndotl >= 0.0 { bias } else { -bias };

        // (then cast the shadow ray from shadow_origin)
        let mut in_shadow = false;
        for obj in objects {
            let h = obj.ray_intersect(&shadow_origin, &l_dir);
            if h.is_intersecting && h.distance < light_dist {
                in_shadow = true;
                break;
            }
        }

        if !in_shadow {
            let ndotl = closest.normal.dot(l_dir).max(0.0);
            let lambert = ndotl * light.intensity * closest.material.albedo;
            out_color = color_add(out_color, color_mul(base, lambert));
        }
    }

    out_color
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Object],
    lights: &[PointLight],
    camera: &Camera,
    texmgr: &TextureManager,           // <-- NEW
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let rd_cam = Vector3::new(screen_x, screen_y, -1.0).normalized();
            let rd_world = camera.basis_change(&rd_cam).normalized();
            let ro_world = camera.eye;

            let pixel_color = cast_ray(&ro_world, &rd_world, objects, lights, texmgr);
            framebuffer.set_current_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracer (Textured)")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::BLACK);
    framebuffer.set_background_color(Color::new(4, 12, 36, 255));

    // Load textures once
    let texmgr = TextureManager::new(&mut window, &raylib_thread); // <-- NEW

    // Escena: un cubo AABB y una esfera con texturas
    let objects: Vec<Object> = vec![
        Object::Cube(Cube {
            min: Vector3::new(-1.0, -1.0, -5.0),
            max: Vector3::new( 1.0,  1.0, -3.0),
            material: Material {
                diffuse: Color::WHITE,//new(200, 180, 140, 255),
                albedo: 0.5,
                texture: Some('#'),  // <-- uses assets/wall3.png per your map
            },
            face_textures: [
                    Some('|'),
                    Some('|'),
                    Some('g'),
                    Some('#'),
                    Some('|'),
                    Some('|'),
                ],
        }),
        Object::Cube(Cube {
            min: Vector3::new(-1.0, -3.0, -5.0),
            max: Vector3::new( 1.0,  -1.0, -3.0),
            material: Material {
                diffuse: Color::WHITE,//new(200, 180, 140, 255),
                albedo: 0.5,
                texture: Some('+'),  // <-- uses assets/wall3.png per your map
            },
            face_textures: [
                    Some('+'),
                    Some('+'),
                    Some('+'),
                    Some('+'),
                    Some('+'),
                    Some('+'),
                ],
        }),
        Object::Sphere(Sphere {
            center: Vector3::new(-1.2, -0.5, -6.0),
            radius: 0.6,
            material: Material {
                diffuse: Color::new(180, 40, 40, 255),
                albedo: 1.0,
                texture: Some('+'),  // <-- assets/wall4.png
            }
        }),
    ];

    let mut lights: Vec<PointLight> = vec![
        PointLight::new(Vector3::new(100.0, 0.0, 0.0), 2.0 ),
        PointLight::new(Vector3::new(2.5, 3.0, -2.0), 1.8 ),
    ];

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let rotation_speed = PI / 100.0;
    let zoom_speed = 1.0;

    while !window.window_should_close() {
        framebuffer.clear();
        if window.is_key_down(KeyboardKey::KEY_LEFT)  { camera.orbit( rotation_speed, 0.0); }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) { camera.orbit(-rotation_speed, 0.0); }
        if window.is_key_down(KeyboardKey::KEY_UP)    { camera.orbit(0.0, -rotation_speed); }
        if window.is_key_down(KeyboardKey::KEY_DOWN)  { camera.orbit(0.0,  rotation_speed); }
        if window.is_key_down(KeyboardKey::KEY_R)     { camera.zoom(zoom_speed); }
        if window.is_key_down(KeyboardKey::KEY_F)     { camera.zoom(-zoom_speed); }

        lights[0].rotate();

        render(&mut framebuffer, &objects, &lights, &camera, &texmgr); // <-- NEW
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}