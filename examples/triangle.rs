//! Usage example of drawing a triangle.
//!
//! Look for `old_school_gfx_glutin_ext` to see what this crate is doing:
//! * Initialise window & gfx.
//! * Resize gfx views.
#[macro_use]
extern crate gfx;

use gfx::{traits::FactoryExt, Device};
use glutin::{
    context::PossiblyCurrentContext,
    surface::{GlSurface, Surface, WindowSurface},
};
use std::{error::Error, num::NonZeroU32};
use winit::{
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::Window,
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
    Ok(EventLoop::new()?.run_app(&mut WinitApp::None)?)
}

enum WinitApp {
    None,
    Resumed(App),
}

impl winit::application::ApplicationHandler for WinitApp {
    fn resumed(&mut self, events: &ActiveEventLoop) {
        events.set_control_flow(ControlFlow::Poll);
        *self = Self::Resumed(App::new(events).unwrap());
    }

    fn window_event(
        &mut self,
        events: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Self::Resumed(app) = self {
            app.window_event(events, event);
        }
    }

    fn about_to_wait(&mut self, _events: &ActiveEventLoop) {
        if let Self::Resumed(App { window, .. }) = self {
            window.request_redraw();
        };
    }
}

struct App {
    window: Window,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
    device: gfx_device_gl::Device,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    dimensions: PhysicalSize<u32>,
    data: pipe::Data<gfx_device_gl::Resources>,
    slice: gfx::Slice<gfx_device_gl::Resources>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    depth_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
}

impl App {
    fn new(events: &ActiveEventLoop) -> Result<Self, Box<dyn Error>> {
        let old_school_gfx_glutin_ext::Init {
            window,
            gl_surface,
            gl_context,
            device,
            mut factory,
            color_view,
            depth_view,
            ..
        } = old_school_gfx_glutin_ext::window_builder(
            events,
            Window::default_attributes()
                .with_title("Triangle")
                .with_inner_size(winit::dpi::PhysicalSize::new(1024, 768)),
        )
        .build::<ColorFormat, DepthFormat>()?;

        let encoder = gfx::Encoder::from(factory.create_command_buffer());

        let pso = factory.create_pipeline_simple(
            include_bytes!("triangle.vs.glsl"),
            include_bytes!("triangle.fs.glsl"),
            pipe::new(),
        )?;

        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: color_view,
        };
        let dimensions = window.inner_size();
        Ok(Self {
            window,
            gl_surface,
            gl_context,
            device,
            encoder,
            dimensions,
            data,
            slice,
            pso,
            depth_view,
        })
    }

    fn window_event(&mut self, elwt: &ActiveEventLoop, event: WindowEvent) {
        let Self {
            window,
            gl_surface,
            gl_context,
            device,
            encoder,
            dimensions,
            data,
            slice,
            pso,
            depth_view,
        } = self;

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => elwt.exit(),
            WindowEvent::RedrawRequested => {
                // handle resizes
                let window_size = window.inner_size();
                if *dimensions != window_size {
                    if let (Some(w), Some(h)) = (
                        NonZeroU32::new(window_size.width),
                        NonZeroU32::new(window_size.height),
                    ) {
                        gl_surface.resize(gl_context, w, h);
                        old_school_gfx_glutin_ext::resize_views(
                            window_size,
                            &mut data.out,
                            depth_view,
                        );
                    }
                    *dimensions = window_size;
                }

                // draw a frame
                encoder.clear(&data.out, CLEAR_COLOR);
                encoder.draw(slice, pso, data);
                encoder.flush(device);
                gl_surface.swap_buffers(gl_context).unwrap();
                device.cleanup();
            }
            _ => (),
        }
    }
}
