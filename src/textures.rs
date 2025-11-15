// textures.rs

use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    images: HashMap<char, Image>,       // Store images for pixel access
    textures: HashMap<char, Texture2D>, // Store GPU textures for rendering
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        // Map characters to texture file paths
        let texture_files = vec![
            ('+', "assets/stone.png"),
            //('-', "assets/wall2.png"),
            ('g', "assets/moss_block.png"),
            ('|', "assets/grass_block_side.png"),
            ('#', "assets/dirt.png"), // default/fallback
        ];

        for (ch, path) in texture_files {
            let mut image = Image::load_image(path).expect(&format!("Failed to load image {}", path));

            // Force a known layout: UNCOMPRESSED_R8G8B8A8
            image.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);

            // Optional, if your UVs expect flipped V:
            // image.flip_vertical();
            let texture = rl.load_texture(thread, path).expect(&format!("Failed to load texture {}", path));
            images.insert(ch, image);
            textures.insert(ch, texture);
        }

        TextureManager { images, textures }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            get_pixel_color(image, x, y)
        } else {
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }
    // textures.rs (add this inside impl TextureManager)
    pub fn sample_uv(&self, ch: char, u: f32, v: f32) -> Color {
        if let Some(img) = self.images.get(&ch) {
            let w = img.width as u32;
            let h = img.height as u32;
            // tile and flip V (common convention)
            let uu = ((u % 1.0) + 1.0) % 1.0;
            let vv = ((v % 1.0) + 1.0) % 1.0;
            let tx = (uu * w as f32).floor() as u32;
            let ty = ((1.0 - vv) * h as f32).floor() as u32;
            self.get_pixel_color(ch, tx.min(w.saturating_sub(1)), ty.min(h.saturating_sub(1)))
        } else {
            Color::WHITE
        }
    }
    pub fn sample_uv_bilinear(&self, ch: char, u: f32, v: f32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            let w = image.width as i32;
            let h = image.height as i32;

            // Wrap and convert to pixel space (centered on pixel centers)
            let uu = ((u % 1.0) + 1.0) % 1.0;
            let vv = ((v % 1.0) + 1.0) % 1.0;

            let x = uu * (w as f32 - 1.0);
            let y = (1.0 - vv) * (h as f32 - 1.0);

            let x0 = x.floor() as i32;
            let y0 = y.floor() as i32;
            let x1 = (x0 + 1).clamp(0, w - 1);
            let y1 = (y0 + 1).clamp(0, h - 1);

            let fx = x - x0 as f32;
            let fy = y - y0 as f32;

            let c00 = get_pixel_color(image, x0, y0);
            let c10 = get_pixel_color(image, x1, y0);
            let c01 = get_pixel_color(image, x0, y1);
            let c11 = get_pixel_color(image, x1, y1);

            // Lerp helpers (operate in linear-ish space; good enough for now)
            let lerp = |a: Color, b: Color, t: f32| -> Color {
                Color::new(
                    (a.r as f32 + (b.r as f32 - a.r as f32) * t).round() as u8,
                    (a.g as f32 + (b.g as f32 - a.g as f32) * t).round() as u8,
                    (a.b as f32 + (b.b as f32 - a.b as f32) * t).round() as u8,
                    (a.a as f32 + (b.a as f32 - a.a as f32) * t).round() as u8,
                )
            };

            let cx0 = lerp(c00, c10, fx);
            let cx1 = lerp(c01, c11, fx);
            return lerp(cx0, cx1, fy);
        }
        Color::WHITE
    }
}

fn get_pixel_color(image: &Image, x: i32, y: i32) -> Color {
    let width = image.width as usize;
    let height = image.height as usize;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return Color::WHITE;
    }

    let x = x as usize;
    let y = y as usize;

    let data_len = width * height * 4;

    unsafe {
        let data = slice::from_raw_parts(image.data as *const u8, data_len);

        let idx = (y * width + x) * 4;

        if idx + 3 >= data_len {
            return Color::WHITE;
        }

        Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }
}