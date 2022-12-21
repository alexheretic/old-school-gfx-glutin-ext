//! Usage example of drawing a triangle.
//!
//! Look for `old_school_gfx_glutin_ext` to see what this crate is doing:
//! * Initialise window & gfx.
//! * Resize gfx views.
#[macro_use]
extern crate gfx;

use gfx::{traits::FactoryExt, Device};
use glutin::surface::GlSurface;
use std::{error::Error, num::NonZeroU32};
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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
    let events = EventLoop::new();

    // Initialise winit window, glutin context & gfx views
    let old_school_gfx_glutin_ext::Init {
        window,
        gl_surface,
        gl_context,
        mut device,
        mut factory,
        color_view,
        mut depth_view,
        ..
    } = old_school_gfx_glutin_ext::window_builder(
        &events,
        WindowBuilder::new()
            .with_title("Triangle")
            .with_inner_size(winit::dpi::PhysicalSize::new(1024, 768)),
    )
    .build::<ColorFormat, DepthFormat>()?;

    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let pso = factory.create_pipeline_simple(
        include_bytes!("triangle.vs.glsl"),
        include_bytes!("triangle.fs.glsl"),
        pipe::new(),
    )?;

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: color_view,
    };
    let mut dimensions = window.inner_size();

    events.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
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
                // handle resizes
                let window_size = window.inner_size();
                if dimensions != window_size {
                    if let (Some(w), Some(h)) = (
                        NonZeroU32::new(window_size.width),
                        NonZeroU32::new(window_size.height),
                    ) {
                        gl_surface.resize(&gl_context, w, h);
                        old_school_gfx_glutin_ext::resize_views(
                            window_size,
                            &mut data.out,
                            &mut depth_view,
                        );
                    }
                    dimensions = window_size;
                }

                // draw a frame
                encoder.clear(&data.out, CLEAR_COLOR);
                encoder.draw(&slice, &pso, &data);
                encoder.flush(&mut device);
                gl_surface.swap_buffers(&gl_context).unwrap();
                device.cleanup();
            }
            _ => (),
        }
    });
}
