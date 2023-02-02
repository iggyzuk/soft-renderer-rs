use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use world::World;

use crate::app::{gui::Framework, timestep::TimeStep};

mod app;
mod graphics;
mod math;
mod world;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;
const RESOLUTION: u32 = 2;
const MS_PER_UPDATE: f32 = 1000.0 / 120.0;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    simple_logger::init_with_level(log::Level::Info).unwrap();

    let width_lowres = WIDTH / RESOLUTION;
    let height_lowres = HEIGHT / RESOLUTION;

    // @todo: learn how winit works and its patterns

    // create the event-loop and input
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    // create the window
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Software Renderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // create the pixels surface texture
    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(width_lowres, height_lowres, surface_texture).unwrap();

        let framework = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        );

        (pixels, framework)
    };

    // create the world and time-step
    let mut world = World::new(width_lowres, height_lowres);

    let mut timestep = TimeStep::new();
    let mut lag = 0.0;

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                framework.resize(size.width, size.height);
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
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            Event::RedrawRequested(_) => {
                world.draw(pixels.get_frame_mut(), 0.1);
                // pixels.render().unwrap();

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if let Err(err) = render_result {
                    log::error!("pixels.render() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}

// @todo: how can we move this into the camera?
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
