//! Usage example of drawing a triangle.
//!
//! This library is the `with_gfx_color_depth`, `init_gfx`, `update_gfx` bits.
#[macro_use]
extern crate gfx;

use gfx::{traits::FactoryExt, Device};
use glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use old_school_gfx_glutin_ext::*;
use std::error::Error;

type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "pos",
        color: [f32; 3] = "color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "target",
    }
}
impl Eq for pipe::Meta {}

const TRIANGLE: [Vertex; 3] = [
    Vertex {
        pos: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [0.0, 0.5],
        color: [0.0, 0.0, 1.0],
    },
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.11, 0.12, 1.0];

pub fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window_config = WindowBuilder::new()
        .with_title("Triangle".to_string())
        .with_inner_size(glutin::dpi::PhysicalSize::new(1024, 768));

    // vvvvvvvvvvv      Initialize gfx      vvvvvvvvvvv
    let (window_ctx, mut device, mut factory, main_color, mut main_depth) =
        glutin::ContextBuilder::new()
            .with_gfx_color_depth::<ColorFormat, DepthFormat>()
            .build_windowed(window_config, &event_loop)?
            .init_gfx::<ColorFormat, DepthFormat>();

    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let pso = factory.create_pipeline_simple(
        include_bytes!("triangle.vs.glsl"),
        include_bytes!("triangle.fs.glsl"),
        pipe::new(),
    )?;

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window_ctx.window().request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical) => {
                    window_ctx.resize(physical);
                    // vvvvvvvvvvv    Update gfx views      vvvvvvvvvvv
                    window_ctx.update_gfx(&mut data.out, &mut main_depth);
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // draw a frame
                encoder.clear(&data.out, CLEAR_COLOR);
                encoder.draw(&slice, &pso, &data);
                encoder.flush(&mut device);
                window_ctx.swap_buffers().unwrap();
                device.cleanup();
            }
            _ => (),
        }
    });
}
