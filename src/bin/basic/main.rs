use core::app::timestep::TimeStep;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::gui::Framework;
use crate::world::World;

pub mod gui;
pub mod world;

const WIDTH: u32 = 64; //1080/8;
const HEIGHT: u32 = 64; //720/8;
const RESOLUTION: u32 = 2;
const TICKS: f32 = 30.0;
const SECONDS_PER_TICK: f32 = 1.0 / TICKS;
const MS_PER_TICK: f32 = SECONDS_PER_TICK * 1000.0;

fn main() {
    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {text:?}");

    let x = Option::map(text, |s| s.len());

    std::env::set_var("RUST_BACKTRACE", "1");

    simple_logger::init_with_level(log::Level::Info).unwrap();

    let width_lowres = WIDTH / RESOLUTION;
    let height_lowres = HEIGHT / RESOLUTION;

    let event_loop = EventLoop::new();

    // create the window
    let window = {
        let size = LogicalSize::new(WIDTH as f64 * 4.0, HEIGHT as f64 * 4.0);
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
        let pixels = Pixels::new(WIDTH * 2, HEIGHT * 2, surface_texture).unwrap();

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

    // start running the loop
    event_loop.run(move |event, _, control_flow| {
        // control flow: poll, poll, poll
        control_flow.set_poll();

        // pass all events to the world for camera movements
        world.handle_event(&event);

        // always pass window events to the gui framework
        match event {
            Event::WindowEvent { ref event, .. } => {
                framework.handle_event(event);
            }
            _ => {}
        }

        match event {
            // match a `close-requested` window event
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            // match a `keyboard-input` window event, only the pressed states
            Event::WindowEvent { event, .. } => match event {
                // resize pixel canvas
                WindowEvent::Resized(size) => {
                    if let Err(_) = pixels.resize_surface(size.width, size.height) {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    framework.resize(size.width, size.height);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } =>
                // match only on specific keys (`1`,`2`,`3`)
                {
                    match virtual_code {
                        VirtualKeyCode::Escape => {
                            control_flow.set_exit();
                        }
                        VirtualKeyCode::Key1 => {
                            println!("1 press");
                        }
                        VirtualKeyCode::Key2 => {
                            println!("2 press");
                        }
                        VirtualKeyCode::Key3 => {
                            println!("3 press");
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // main loop
                let delta = timestep.delta();

                lag += delta;

                while lag >= MS_PER_TICK {
                    lag -= MS_PER_TICK;
                    world.update(SECONDS_PER_TICK);
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                world.draw(pixels.frame_mut(), 0.1);

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // // Render egui
                    // framework.render(encoder, render_target, context);

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
