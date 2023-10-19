# Software Renderer in Rust

Every pixel is drawn with love!

- Linear Algebra & Math
- Depth Buffer
- Shadow Mapping
- Camera Movement
- Mesh & Texture Loading
- Triangle Clipping
- Triangle Rasterization
- World with Instances
- Timestep Simulation

![4x](/screenshots/screen_1.jpg)
![1x](/screenshots/screen_2.jpg)

## Run Binary

`cargo run --bin basic --release`

## Run Tests

`cargo test --package core`

## Controls

Move: WASD + QE
Look: ← ↑ → ↓

## Checklist

1. bitmap with various format f32 too for bitmap of depth or a u32 (4 bytes)
2. remove commented code
3. pass light direction to gradients
4. remove pub in unnecessary places