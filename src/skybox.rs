use std::f32::consts::PI;

// sky.rs
use raylib::prelude::*;

use crate::{color::lerp_color, light::PointLight};

pub fn procedural_sky(dir: Vector3) -> Vector3 {
    let d = dir.normalized();
    let t = (d.y + 1.0) * 0.5; // map y [-1,1] → [0,1]

    let green = Vector3::new(0.1, 0.6, 0.2); // grass green
    let white = Vector3::new(1.0, 1.0, 1.0); // horizon haze
    let blue = Vector3::new(0.3, 0.5, 1.0);  // sky blue

    if t < 0.54 {
        // Bottom → fade green to white
        let k = t / 0.55;
        green * (1.0 - k) + white * k
    } else if t < 0.55 {
        // Around horizon → mostly white
        white
    } else if t < 0.8 {
        // Fade white to blue
        let k = (t - 0.55) / (0.25);
        white * (1.0 - k) + blue * k
    } else {
        // Upper sky → solid blue
        blue
    }
}

pub struct Sky {
    pub time: f32,           // 0..1 = fraction of the day
    pub day_length: f32,          // seconds per full day/night cycle
    pub sun: PointLight,
    pub moon: PointLight,
    pub ambient: f32,
}

impl Sky {
    pub fn new() -> Self {
        let time = 0.0_f32;             // 0..1 = fraction of the day
        let day_length = 20.0_f32;          // seconds per full day/night cycle
        let sun = PointLight::new(Vector3::new(0.0, 0.0, -1000.0), 0.0, Color::BLACK, None);
        let moon = PointLight::new(Vector3::new(0.0, 0.0, 1000.0), 0.0, Color::new(200, 210, 255, 255), None);
        Sky {time, day_length, sun, moon, ambient: 0.0}
    }

    pub fn update_sky(&mut self, dt: f32) {
        self.time = (self.time + dt / self.day_length) % 1.0;     // wrap [0,1)
        // Angle over the day: 0..2π
        let theta = self.time * 2.0 * PI;

        // Direction of the sun in world space
        // We’ll rotate in the Y–Z plane: y = height, z = "forward/back"
        let sun_dir = Vector3::new(0.0, theta.sin(), theta.cos()); // y = elevation

        // We'll place the sun far away along *opposite* direction (like it shines towards origin)
        let sun_distance = 1000.0;
        let sun_pos = -sun_dir * sun_distance;

        // "height" of the sun above horizon
        let sun_height = sun_dir.y; // [-1,1], >0 = above ground

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

        // Moon: opposite direction
        let moon_dir = -sun_dir;
        let moon_pos = -moon_dir * sun_distance;

        // Moon visible when sun below horizon
        let moon_visibility = (-sun_height).max(0.0); // >0 at night
        let moon_intensity = moon_visibility * 0.4;   // much dimmer than sun
        let moon_color = Color::new(200, 210, 255, 255); // cold pale blue

        self.sun = PointLight::new(sun_pos, sun_intensity, sun_color, None);

        self.moon = PointLight::new(moon_pos, moon_intensity, moon_color, None);

        self.ambient = 0.02
        + 0.25 * sun_visibility   // brightens scene in day
        + 0.05 * moon_visibility; // slight night ambient
    }

}