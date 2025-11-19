use std::f32::consts::PI;

// sky.rs
use raylib::prelude::*;

use crate::{color::lerp_color, light::PointLight};

pub struct Sky {
    pub time: f32,           // 0..1 = fraction of the day
    pub elapsed: f32,
    pub day_length: f32,          // seconds per full day/night cycle
    pub sun: PointLight,
    pub moon: PointLight,
    pub ambient: f32,
}

impl Sky {
    pub fn new() -> Self {
        let time = 0.0_f32;             // 0..1 = fraction of the day
        let day_length = 20.0_f32;          // seconds per full day/night cycle
        let sun = PointLight::new(Vector3::new(-1000.0, 0.0, 0.0), 0.0, Color::BLACK, None);
        let moon = PointLight::new(Vector3::new(1000.0, 0.0, 0.0), 0.0, Color::new(200, 210, 255, 255), None);
        Sky {time, elapsed: 0.0, day_length, sun, moon, ambient: 0.0}
    }

    pub fn update_sky(&mut self, dt: f32) {
        self.elapsed += dt;
        self.time = (self.time + dt / self.day_length) % 1.0;     // wrap [0,1)
        // Angle over the day: 0..2π
        let theta = self.time * 2.0 * PI;
        self.sun.rotate(theta);
        self.moon.rotate(theta + PI);

        // "height" of the sun above horizon
        let sun_height = theta.sin(); // [-1,1], >0 = above ground

        // Visibility: only when above horizon
        let sun_visibility = sun_height.max(0.0); // 0 at night, 1 at noon-ish

        // Intensity stronger near noon
        let sun_intensity = sun_visibility * 3.0; // tweak

        // t_sun_color ~ 0 at horizon, ~1 at high noon
        let t_sun_color = sun_visibility.powf(0.5);
        let sun_color = if sun_visibility > 0.0 {
            let sunrise = Color::new(255, 170, 90, 255); // orange
            let noon    = Color::new(255, 255, 240, 255); // slightly warm white
            lerp_color(sunrise, noon, t_sun_color)
        } else {
            Color::BLACK
        };

        // Moon visible when sun below horizon
        let moon_visibility = (-sun_height).max(0.0); // >0 at night
        let moon_intensity = moon_visibility * 0.4;   // much dimmer than sun
        let moon_color = Color::new(200, 210, 255, 255); // cold pale blue

        self.sun.set_intensity(sun_intensity);
        self.sun.set_current_color(sun_color);
        self.moon.set_intensity(moon_intensity);
        self.moon.set_current_color(moon_color);

        self.ambient = 0.02
        + 0.25 * sun_visibility   // brightens scene in day
        + 0.05 * moon_visibility; // slight night ambient
    }

    pub fn procedural_sky(&self, dir: Vector3) -> Vector3 {
        let d = dir.normalized();
        let t = (d.y + 1.0) * 0.5; // map y [-1,1] → [0,1]

        // ---------- 1) DAY GRADIENT ----------
        let green = Vector3::new(0.1, 0.6, 0.2); // grass green
        let white = Vector3::new(1.0, 1.0, 1.0); // horizon haze
        let blue  = Vector3::new(0.3, 0.5, 1.0); // sky blue

        let day_color = if t < 0.54 {
            // Bottom → fade green to white
            let k = t / 0.55;
            green * (1.0 - k) + white * k
        } else if t < 0.55 {
            // Around horizon → mostly white
            white
        } else if t < 0.8 {
            // Fade white to blue
            let k = (t - 0.55) / 0.25;
            white * (1.0 - k) + blue * k
        } else {
            // Upper sky → solid blue
            blue
        };

        // ---------- 2) NIGHT GRADIENT ----------
        let night_ground   = Vector3::new(0.01, 0.03, 0.05); // near "ground"
        let night_horizon  = Vector3::new(0.02, 0.05, 0.10); // horizon glow
        let night_zenith   = Vector3::new(0.0,  0.0,  0.08); // deep night sky

        let night_color = if t < 0.54 {
            let k = t / 0.55;
            night_ground * (1.0 - k) + night_horizon * k
        } else if t < 0.55 {
            night_horizon
        } else if t < 0.8 {
            let k = (t - 0.55) / 0.25;
            night_horizon * (1.0 - k) + night_zenith * k
        } else {
            night_zenith
        };

        // ---------- 3) TIME → SUN & MOON INFO ----------
        let theta = self.time * 2.0 * PI;
        let sun_dir = Vector3::new(theta.cos(), theta.sin(), 0.0);
        let moon_dir = -sun_dir;

        let sun_height = sun_dir.y;          // [-1,1]
        let sun_visibility  = sun_height.max(0.0);      // 0 at/below horizon, 1 at noon
        let moon_visibility = (-sun_height).max(0.0);   // 0 at/above horizon, 1 at midnight

        // Day vs night blend for the base sky
        let day_factor   = sun_visibility;
        let night_factor = 1.0 - day_factor;

        let mut color = day_color * day_factor + night_color * night_factor;

        // ---------- 4) SUN DISC ----------
        // Use angular distance via vector difference (no acos needed).
        // When d == sun_dir, dist2 = 0. Larger angle ⇒ larger dist2.
        let sun_dist2 = (d - sun_dir).length().powi(2);

        // Tune these: "radius" in direction-space, not world units
        let sun_outer = 0.12; // edge of sun glow
        let sun_inner = 0.012; // crisp core

        let sun_intensity = if sun_visibility > 0.0 {
            // Map dist2 to [0,1] disc factor using smoothstep
            let dist = sun_dist2.sqrt();
            let core = 1.0 - smoothstep(0.0, sun_inner, dist);   // bright center
            let halo = 1.0 - smoothstep(sun_inner, sun_outer, dist); // soft falloff

            // Sun brightness scales with visibility (higher when high in the sky)
            let brightness = (core * 2.0 + halo) * sun_visibility;

            brightness
        } else {
            0.0
        };

        if sun_intensity > 0.0 {
            // Sun color: hot yellow-white
            let sun_col = Vector3::new(1.0, 0.95, 0.8);
            color += sun_col * sun_intensity;
        }

        // ---------- 5) MOON DISC ----------
        let moon_dist2 = (d - moon_dir).length().powi(2);

        let moon_outer = 0.048;
        let moon_inner = 0.012;

        let moon_intensity = if moon_visibility > 0.0 {
            let dist = moon_dist2.sqrt();
            let core = 1.0 - smoothstep(0.0, moon_inner, dist);
            let halo = 1.0 - smoothstep(moon_inner, moon_outer, dist);

            let brightness = (core * 1.5 + halo * 0.8) * moon_visibility * 0.8;
            brightness
        } else {
            0.0
        };

        if moon_intensity > 0.0 {
            // Moon color: pale blue-white
            let moon_col = Vector3::new(0.85, 0.9, 1.0);
            color += moon_col * moon_intensity;
        }

        // Clamp to [0,1] so we don't blow out
        Vector3::new(
            color.x.clamp(0.0, 1.0),
            color.y.clamp(0.0, 1.0),
            color.z.clamp(0.0, 1.0),
        )
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}