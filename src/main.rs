// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

use raylib::prelude::*;
use std::f32::consts::PI;
use rayon::prelude::*;

mod framebuffer;
mod ray_intersect;
mod material;
mod camera;
mod textures;
mod light;
mod object;
mod color;
mod math;
mod skybox;
mod sprites;

use framebuffer::Framebuffer;
use ray_intersect::{RayIntersect, Hit};
use material::Material;
use camera::Camera;
use textures::TextureManager;
use light::PointLight;
use object::Object;

use crate::{color::*, light::build_lights_from_objects, material::*, math::*, object::sample_objects, skybox::*, sprites::{SpriteSystem, render_sprites}};

const MAX_DEPTH: u32 = 4;
// -------- trazado con Lambert + sombra --------
pub fn cast_ray(
    ro: &Vector3,
    rd: &Vector3,
    objects: &[Object],
    lights: &[PointLight],
    texmgr: &TextureManager,
    sky: &Sky,
    depth: u32,
) -> Color {
    let ambient = sky.ambient;
    let default = sky.procedural_sky(rd.clone());
    if depth >= MAX_DEPTH {
        //return Color::new(4, 12, 36, 255); // background
        return linear_to_srgb(default.x, default.y, default.z);
    }
    // Buscar el hit más cercano
    let mut closest = Hit::no_hit();
    for (oid, obj) in objects.iter().enumerate() {
        let h = obj.ray_intersect(ro, rd, oid);
        if h.is_intersecting && h.distance < closest.distance {
            closest = h;
        }
    }
    if !closest.is_intersecting {
        //return Color::new(4, 12, 36, 255);
        return linear_to_srgb(default.x, default.y, default.z);
    }

    // Pick texture id (per-face or per-material, depending on your setup)
    let tex_id = closest.tex_id.or(closest.material.texture);
    let m = closest.material;

    // Base color in sRGB
    let base_srgb = if let Some(ch) = tex_id {
        let u = closest.uv.x;
        let mut v = closest.uv.y;

        // If this material is animated (frames stacked vertically)
        if m.anim_frames > 1 && m.anim_fps > 0.0 {
            // time in seconds
            let t_anim = sky.elapsed; 

            let frame_f = (t_anim * m.anim_fps).floor();
            let frame_idx = (frame_f as u32) % m.anim_frames;

            let frame_h = 1.0 / m.anim_frames as f32;

            // v in [0,1] inside the frame → shift into the atlas
            v = v.clamp(0.0, 1.0);
            v = frame_idx as f32 * frame_h + v * frame_h;
        }

        texmgr.sample_uv_bilinear(ch, u, v)
    } else {
        m.diffuse
    };

    // Convert to linear for lighting
    let (br, bg, bb) = srgb_to_linear(base_srgb);

    //let ambient = 0.05;

    let mut lr = br * ambient * m.albedo;
    let mut lg = bg * ambient * m.albedo;
    let mut lb = bb * ambient * m.albedo;

    // view direction (towards camera)
    let view_dir = (*ro - closest.point).normalized();

    for light in lights {
        let (lr_l, lg_l, lb_l) = srgb_to_linear(light.color);
        let to_light = light.position - closest.point;
        let light_dist = to_light.length();
        let l_dir = to_light / light_dist;

        // Shadow ray with transparency-aware visibility
        let ndotl_raw = closest.normal.dot(l_dir).clamp(-1.0, 1.0);
        let bias = 5e-3 + 5e-3 * (1.0 - ndotl_raw.abs());
        let shadow_origin = closest.point + closest.normal * if ndotl_raw >= 0.0 { bias } else { -bias };

        let mut light_visibility = 1.0_f32;
        for (obj_index, obj) in objects.iter().enumerate() {
            if Some(obj_index) == light.emitter_index {
                continue;
            }
            let h = obj.ray_intersect(&shadow_origin, &l_dir, obj_index);
            if h.is_intersecting && h.distance < light_dist {
                let mat_blocker = h.material;
                // If the blocker is transparent, let some light through
                if mat_blocker.transparency > 0.0 {
                    light_visibility *= mat_blocker.transparency.clamp(0.0, 1.0);
                    // With our single-hit intersection we can't gather multiple layers,
                    // so we just attenuate once and stop.
                    break;
                } else {
                    // Opaque blocker: full shadow
                    light_visibility = 0.0;
                    break;
                }
            }
        }

        if light_visibility > 0.0 {
            // Diffuse (Lambert)
            let cid = closest.obj_id;
            let ndotl = ndotl_raw.max(0.0);
            let diff = light.intensity * m.albedo * light_visibility * if Some(cid) != light.emitter_index {ndotl} else {ndotl_raw.abs()};

            lr += br * lr_l * diff;
            lg += bg * lg_l * diff;
            lb += bb * lb_l * diff;

            // Specular (Phong)
            if m.specular_strength > 0.0 {
                let reflect_dir = reflect(-l_dir, closest.normal).normalized();
                let rv = reflect_dir.dot(view_dir).max(0.0);
                let mut spec_factor = rv.powf(m.shininess) * m.specular_strength * light.intensity;
                spec_factor *= light_visibility;

                lr += lr_l * spec_factor;
                lg += lg_l * spec_factor;
                lb += lb_l * spec_factor;
            }
        }
    }

    let eps = 1e-3;
    // --- Fresnel-based mixing ---
    // cos_theta: angle between view direction and surface normal
    // rd points TOWARDS the scene, so view dir is -rd.
    let cos_theta = (-rd.dot(closest.normal)).max(0.0);

    // Base reflectivity at normal incidence (f0)
    // Option A: use your material.reflectivity as f0 (0..1)
    let f0 = m.reflectivity.clamp(0.0, 1.0);

    // Option B (more physical for dielectrics):
    // let f0 = f0_from_ior(m.ior);

    // Fresnel reflection factor depending on angle
    let fresnel = fresnel_schlick(cos_theta, f0);

    // kr: reflection factor (angle-dependent)
    // kt: transparency factor (constant for now)
    let kr = fresnel;
    let kt = m.transparency.clamp(0.0, 1.0);

    // kd: leftover part for local (diffuse+specular) lighting
    let mut kd = 1.0 - kr - kt;
    if kd < 0.0 { kd = 0.0; }

    // Start final color with local lighting scaled by kd
    let mut fr = lr * kd;
    let mut fg = lg * kd;
    let mut fb = lb * kd;

    // --- Reflection contribution ---
    if kr > 0.0 {
        let refl_dir = reflect(*rd, closest.normal).normalized();
        let refl_origin = closest.point + closest.normal * eps;
        let refl_color = cast_ray(&refl_origin, &refl_dir, objects, lights, texmgr, &sky, depth + 1);
        let (rr, rg, rb) = srgb_to_linear(refl_color);

        fr += rr * kr;
        fg += rg * kr;
        fb += rb * kr;
    }

    // --- Refraction contribution ---
    if kt > 0.0 {
        if let Some(refr_dir) = refract(*rd, closest.normal, 1.0, m.ior) {
            let refr_origin = closest.point - closest.normal * eps; // slightly inside
            let refr_color = cast_ray(&refr_origin, &refr_dir.normalized(), objects, lights, texmgr, &sky, depth + 1);
            let (tr, tg, tb) = srgb_to_linear(refr_color);

            fr += tr * kt;
            fg += tg * kt;
            fb += tb * kt;
        }
    }

    // Convert back to sRGB and return
    return linear_to_srgb(fr, fg, fb);
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Object],
    lights: &[PointLight],
    camera: &Camera,
    texmgr: &TextureManager,
    sky: &Sky,
) {
    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;

    let w_f = width as f32;
    let h_f = height as f32;
    let aspect_ratio = w_f / h_f;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    // 1) Temporary image buffer for this frame
    let mut pixels = vec![Color::BLACK; width * height];

    // 2) Parallel over rows
    pixels
        .par_chunks_mut(width)   // each chunk = one row [x=0..width-1]
        .enumerate()
        .for_each(|(y, row)| {
            let y = y as u32;
            for x in 0..width as u32 {
                let screen_x = (2.0 * x as f32) / w_f - 1.0;
                let screen_y = -(2.0 * y as f32) / h_f + 1.0;

                let screen_x = screen_x * aspect_ratio * perspective_scale;
                let screen_y = screen_y * perspective_scale;

                let rd_cam = Vector3::new(screen_x, screen_y, -1.0).normalized();
                let rd_world = camera.basis_change(&rd_cam).normalized();
                let ro_world = camera.eye;

                let color = cast_ray(&ro_world, &rd_world, objects, lights, texmgr, sky, 0);
                row[x as usize] = color;
            }
        });

    // 3) Copy into framebuffer (single-threaded)
    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let idx = y as usize * width + x as usize;
            framebuffer.set_current_color(pixels[idx]);
            framebuffer.set_pixel(x, y);
        }
    }
}

fn main() {
    let window_width = 500;
    let window_height = 250;

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
    let objects: Vec<Object> = sample_objects();
    let mut lights: Vec<PointLight> = build_lights_from_objects(&objects);//vec![
    
    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let rotation_speed = PI / 100.0;
    let zoom_speed = 1.0;

    let mut sprite_system = SpriteSystem::new(
        Vector3::new(1.0, 0.0, -1.0), 
        Vector3::new(2.0, 3.0, 1.0)
    );

    let mut sky = Sky::new();

    while !window.window_should_close() {
        framebuffer.clear();
        if window.is_key_down(KeyboardKey::KEY_LEFT)  { camera.orbit( rotation_speed, 0.0); }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) { camera.orbit(-rotation_speed, 0.0); }
        if window.is_key_down(KeyboardKey::KEY_UP)    { camera.orbit(0.0, -rotation_speed); }
        if window.is_key_down(KeyboardKey::KEY_DOWN)  { camera.orbit(0.0,  rotation_speed); }
        if window.is_key_down(KeyboardKey::KEY_R)     { camera.zoom(zoom_speed); }
        if window.is_key_down(KeyboardKey::KEY_F)     { camera.zoom(-zoom_speed); }

        let dt = window.get_frame_time();
        sky.update_sky(dt);
        lights.push(sky.sun);lights.push(sky.moon);
        render(&mut framebuffer, &objects, &lights, &camera, &texmgr, &sky); // <-- NEW
        
        sprite_system.update(dt, &camera, &objects);
        let fov = PI/3.0;
        render_sprites(&mut framebuffer, &sprite_system.sprites, &camera, &texmgr, fov);
        lights.pop(); lights.pop();
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}