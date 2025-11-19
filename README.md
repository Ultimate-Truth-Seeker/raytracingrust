# Minecraft‑Inspired Raytraced Diorama  
*A fully custom Rust + Raylib CPU raytracer*

https://github.com/Ultimate-Truth-Seeker/raytracingrust

---

## 🌄 Project Overview

This project is a fully custom **software raytracer** written in **Rust**, inspired by the blocky aesthetic of **Minecraft** but built from scratch with advanced rendering features.

It renders a diorama scene containing blocks, emissive lamps, an animated nether portal, imported OBJ models, procedural sky, sprite effects, and a full material system — all raytraced on the CPU.

In addition, it includes:

- **Background music** (looping while the window is open)
- **Orbiting & zooming camera** (left/right/up/down + zoom keys)
- **Animated textures and sprites**
- **Multithreaded rendering** using Rayon
- **Custom OBJ loader**
- **Day-night cycle with sun and moon rendering**

---

## 🧭 Features

### 🟦 1. Minecraft‑Inspired Diorama  
A procedural, handcrafted mini‑world built from cubes and custom materials, rendered entirely through ray‑triangle and ray‑box intersection.

Includes:
- Terrain blocks (grass, dirt, stone)
- Obsidian nether portal
- Emissive lamps
- Imported mesh decorations

---

### ☀️ 2. Procedural Skybox with Day/Night Cycle

The raytracer contains a **procedural skybox shader** that varies based on:
- Ray direction  
- Time of day  
- Sun direction  
- Moon direction  

The sky color transitions smoothly through:
- Dawn  
- Noon  
- Sunset  
- Night sky  

It also renders:
- **Sun disc**  
- **Moon disc**  
- Horizon blending  
- Ambient light variations based on time  

---

### 🌀 3. Animated Nether Portal

The portal is a **custom animated quad**, textured using:
- A vertically stacked PNG animation sheet  
- Time‑based frame indexing  
- UV warping  
- Sprite effects on top  
- Adjustable FPS and scaling  

Portal frame blocks use a **custom emissive portal material**.

---

### ⭐ 4. Sprite System  
The scene supports **floating sprites** that:
- Spawn randomly in a region  
- Move over time  
- Fade/appear randomly  
- Only render if the sprite is visible from camera (occlusion tested by raytrace)  

Useful for:  
- Magical particles near the portal  
- Ambient effects  

---

### 🎶 5. Background Music

A looping audio track plays while the window is open using Raylib's audio engine.

---

### 📦 6. OBJ Model Loading

The engine contains a simple **OBJ loader** that supports:
- Vertex positions  
- Face indices  
- Triangle rebuilding  
- Mesh translation + scaling  
- Per-face shading  

Imported models render as regular objects in the scene with full lighting.

---

### 🧱 7. Material System (8 Materials Total)

Every object has a `Material` with parameters:

| Property | Description |
|----------|-------------|
| **albedo** | Base reflectance color |
| **specular_strength** | Mirror-like highlights |
| **shininess** | Glossiness exponent |
| **reflectivity** | Reflective ratio for Fresnel |
| **transparency** | Fraction of refracted (transmitted) light |
| **ior** | Index of refraction (for glass, water, etc.) |
| **texture** | Optional per-face or per-object texture |
| **emission** | Emissive color (light produced) |

### Implemented Materials:

1. **Default**
2. **Grass** (textured top, dirt sides)
3. **Dirt**
4. **Stone**
5. **Obsidian** (dark, reflective)
6. **Glass**  
   - Implements **reflection + refraction**  
   - Uses **Schlick Fresnel**  
   - Transparent with adjustable IOR  
7. **Portal**
   - Fully emissive animation
   - No shadow casting on itself
8. **Lamp**
   - Strong emissive material  
   - Lights nearby blocks  

Lighting is calculated with:
- Lambert diffuse
- Blinn‑Phong specular
- Fresnel reflection
- Hard shadows
- Emissive additive contribution
- Transparency ray continuation

---

### 🎥 8. Camera Controls

The **Camera** supports:

- **Orbiting** around the diorama (arrow keys)
- **Zoom in/out** with smooth distance constraints (`R`/`F`)
- **Look direction** maintained by a stable forward/right/up basis

Camera math uses spherical coordinates.
The camera has a limit for zooming in at **10.0 units**, so as to not slow down the scene.

---

### ⚙️ 9. Multithreaded Raytracing

The scene is rendered using Rayon:

- Each scanline rendered in parallel  
- Each pixel computes `cast_ray()` independently  
- Thread-safe texture access  
- Fast realtime preview at medium resolution  

---

## 🏗 Project Structure (Simplified)

```
src/
  main.rs
  camera.rs
  framebuffer.rs
  ray_intersect.rs
  material.rs
  textures.rs
  skybox.rs
  sprites.rs
  light.rs
  color.rs
  math.rs
  object/
    mod.rs
    cube.rs
    sphere.rs
    mesh.rs
    animated_quad.rs
    obj.rs
assets/
  grass.png
  dirt.png
  stone.png
  portal_anim.png
  lamp.png
  music.ogg
  meshes/
      tree.obj
      decorations.obj
README.md
```

---

## 🚀 Running the Project

```sh
cargo run --release
```

---

## 📸 Screenshots  
TBD

---

## 🎬 Demo Video  
![Demo Video](assets/demo.gif)

---

## 📜 License  
MIT License

---
