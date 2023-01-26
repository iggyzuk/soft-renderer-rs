use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use world::World;

use crate::app::timestep::TimeStep;

mod app;
mod graphics;
mod math;
mod world;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const RESOLUTION: u32 = 2;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    log::info!("starting...");

    let event_loop = EventLoop::new();

    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Software Renderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // @todo: move this into the renderer but get the physical size first.
    let pixels = {
        let window_size = window.inner_size();
        // let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH / RESOLUTION, HEIGHT / RESOLUTION, surface_texture).unwrap()
    };

    let mut world = World::new(WIDTH / RESOLUTION, HEIGHT / RESOLUTION, pixels);

    const MS_PER_UPDATE: f32 = 1000.0 / 120.0;

    let mut timestep = TimeStep::new();
    let mut lag = 0.0;

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            let delta = timestep.delta();

            lag += delta;

            while lag >= MS_PER_UPDATE {
                lag -= MS_PER_UPDATE;

                if input.update(&event) {
                    camera_controls(&mut input, &mut world, MS_PER_UPDATE / 1000.0);
                }

                world.update(MS_PER_UPDATE / 1000.0);
            }

            window.request_redraw();
        }

        match event {
            Event::WindowEvent { .. } => {}
            Event::RedrawRequested(_) => {
                world.draw(0.1);
            }
            _ => (),
        }
    });
}

fn camera_controls(input: &mut WinitInputHelper, world: &mut World, dt: f32) {
    if input.key_held(VirtualKeyCode::W) {
        world.input(world::WorldInputEvent::MoveForward, dt);
    }
    if input.key_held(VirtualKeyCode::S) {
        world.input(world::WorldInputEvent::MoveBack, dt);
    }
    if input.key_held(VirtualKeyCode::E) {
        world.input(world::WorldInputEvent::MoveUp, dt);
    }
    if input.key_held(VirtualKeyCode::Q) {
        world.input(world::WorldInputEvent::MoveDown, dt);
    }
    if input.key_held(VirtualKeyCode::A) {
        world.input(world::WorldInputEvent::MoveLeft, dt);
    }
    if input.key_held(VirtualKeyCode::D) {
        world.input(world::WorldInputEvent::MoveRight, dt);
    }

    if input.key_held(VirtualKeyCode::Up) {
        world.input(world::WorldInputEvent::LookUp, dt);
    }
    if input.key_held(VirtualKeyCode::Down) {
        world.input(world::WorldInputEvent::LookDown, dt);
    }
    if input.key_held(VirtualKeyCode::Left) {
        world.input(world::WorldInputEvent::LookLeft, dt);
    }
    if input.key_held(VirtualKeyCode::Right) {
        world.input(world::WorldInputEvent::LookRight, dt);
    }
}
