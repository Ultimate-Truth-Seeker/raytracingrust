// sprites.rs
use raylib::prelude::*;
use rand::Rng;
use crate::{camera::Camera, object::Object, ray_intersect::{Hit, RayIntersect}};
use crate::{framebuffer::Framebuffer, textures::TextureManager};

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub position: Vector3,
    pub velocity: Vector3,
    pub size_world: f32,   // physical size (square) in world units
    pub age: f32,
    pub lifetime: f32,
    pub tex_id: char,      // which texture in TextureManager to use
    pub visible: bool,
}

pub struct SpriteSystem {
    pub sprites: Vec<Sprite>,
    pub region_min: Vector3,
    pub region_max: Vector3,
}

impl SpriteSystem {
    pub fn new(region_min: Vector3, region_max: Vector3) -> Self {
        let mut sprites = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..25 {
            let pid = rng.gen_range(0..=7);
            sprites.push(respawn(region_min, region_max, match pid {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                _ => '0'
            }));
        }
        Self {
            sprites,
            region_min,
            region_max,
        }
    }

    pub fn update(&mut self, dt: f32, camera: &Camera, objects: &[Object]) {
        for s in &mut self.sprites {
            s.age += dt;
            if s.age > s.lifetime {
                // respawn as a new random sprite
                *s = respawn(self.region_min, self.region_max, s.tex_id);
                continue;
            }

            s.position += s.velocity * dt;

            // Optionally keep inside region (bounce or wrap)
            if s.position.x < self.region_min.x || s.position.x > self.region_max.x
                || s.position.y < self.region_min.y || s.position.y > self.region_max.y
                || s.position.z < self.region_min.z || s.position.z > self.region_max.z
            {
                *s = respawn(self.region_min, self.region_max, s.tex_id);
            }

            // Update visibility based on occlusion
            s.visible = sprite_visible(camera, s.position, objects);
        }
    }
}

fn sprite_visible(camera: &Camera, sprite_pos: Vector3, objects: &[Object]) -> bool {
        let ro = camera.eye;
        let to_sprite = sprite_pos - ro;
        let dist = to_sprite.length();
        if dist <= 1e-4 {
            return true;
        }

        let rd = to_sprite / dist;

        let mut closest = Hit::no_hit();
        for (obj_id, obj) in objects.iter().enumerate() {
            
            let h = obj.ray_intersect(&ro, &rd, obj_id);
            if h.is_intersecting && h.distance < closest.distance {
                 if h.material.transparency < 0.5 {
                    closest = h;
                }
            }
        }

        // If no hit, sprite is visible
        if !closest.is_intersecting {
            return true;
        }

        // If closest hit is farther than sprite, sprite is in front
        closest.distance >= dist - 1e-3
}

fn respawn(rmin: Vector3, rmax: Vector3, tex_id: char) -> Sprite {
        let mut rng = rand::thread_rng();

        let rx = rng.r#gen::<f32>();
        let ry = rng.r#gen::<f32>();
        let rz = rng.r#gen::<f32>();

        let pos = Vector3::new(
            rmin.x + (rmax.x - rmin.x) * rx,
            rmin.y + (rmax.y - rmin.y) * ry,
            rmin.z + (rmax.z - rmin.z) * rz,
        );

        let vx = (rng.r#gen::<f32>() - 0.5) * 2.0;
        let vy = (rng.r#gen::<f32>() - 0.5) * 2.0;
        let vz = (rng.r#gen::<f32>() - 0.5) * 2.0;
        let vel = Vector3::new(vx, vy, vz) * 0.5;

        let lifetime = rng.gen_range(1.0..4.0);

        Sprite {
            position: pos,
            velocity: vel,
            size_world: 0.2,
            age: 0.0,
            lifetime,
            tex_id,
            visible: true,
        }
    }


pub fn project_to_screen(
    camera: &Camera,
    p_world: &Vector3,
    width: f32,
    height: f32,
    fov: f32,
) -> Option<(i32, i32, f32)> {
    let aspect = width / height;
    let perspective_scale = (fov * 0.5).tan();

    let p_cam = camera.world_to_camera(p_world);

    // Behind camera?
    if p_cam.z >= 0.0 {
        return None;
    }

    // NDC coordinates
    let x_ndc = (p_cam.x / -p_cam.z) / (aspect * perspective_scale);
    let y_ndc = (p_cam.y / -p_cam.z) / (perspective_scale);

    if x_ndc < -1.0 || x_ndc > 1.0 || y_ndc < -1.0 || y_ndc > 1.0 {
        return None;
    }

    let sx = ((x_ndc + 1.0) * 0.5) * width;
    let sy = ((1.0 - y_ndc) * 0.5) * height;

    Some((sx as i32, sy as i32, -p_cam.z))
}

pub fn render_sprites(
    framebuffer: &mut Framebuffer,
    sprites: &[Sprite],
    camera: &Camera,
    texmgr: &TextureManager,
    fov: f32,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    for s in sprites {
        if !s.visible {
            continue;
        }

        if let Some((sx, sy, depth)) = project_to_screen(camera, &s.position, width, height, fov) {
            // approximate size in pixels from world size + depth
            let perspective_scale = (fov * 0.5).tan();
            let pixel_size = (s.size_world / depth)
                * (height / (2.0 * perspective_scale)); // rough: world→screen

            let half = pixel_size as i32 / 2;
            let min_x = (sx - half).max(0);
            let max_x = (sx + half).min(framebuffer.width as i32 - 1);
            let min_y = (sy - half).max(0);
            let max_y = (sy + half).min(framebuffer.height as i32 - 1);

            let tex = texmgr.get_texture(s.tex_id);
            if tex.is_none() { continue; }
            let tex = tex.unwrap();
            let tw = tex.width as f32;
            let th = tex.height as f32;

            for py in min_y..=max_y {
                for px in min_x..=max_x {
                    // local sprite UV
                    let u = (px - min_x) as f32 / (max_x - min_x + 1) as f32;
                    let v = (py - min_y) as f32 / (max_y - min_y + 1) as f32;

                    let tx = (u * (tw - 1.0)) as u32;
                    let ty = (v * (th - 1.0)) as u32;

                    let mut col = texmgr.get_pixel_color(s.tex_id, tx, ty);

                    // Simple alpha test (no blending): skip transparent pixels
                    if col.a == 0 {
                        continue;
                    } else {
                        col = Color::MAGENTA;
                    }

                    framebuffer.set_current_color(col);
                    framebuffer.set_pixel(px as u32, py as u32);
                }
            }
        }
    }
}