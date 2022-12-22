//! Extensions for [glutin](https://crates.io/crates/glutin) to initialize & update old school
//! [gfx](https://crates.io/crates/gfx). _An alternative to gfx_window_glutin_.
//!
//! # Example
//! ```no_run
//! type ColorFormat = gfx::format::Srgba8;
//! type DepthFormat = gfx::format::DepthStencil;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_loop = winit::event_loop::EventLoop::new();
//! let window_builder = winit::window::WindowBuilder::new();
//!
//! // Initialise winit window, glutin context & gfx views
//! let old_school_gfx_glutin_ext::Init {
//!     // winit window
//!     window,
//!     // glutin bits
//!     gl_config,
//!     gl_surface,
//!     gl_context,
//!     // gfx bits
//!     mut device,
//!     mut factory,
//!     mut color_view,
//!     mut depth_view,
//!     ..
//! } = old_school_gfx_glutin_ext::window_builder(&event_loop, window_builder)
//!     .build::<ColorFormat, DepthFormat>()?;
//!
//! # let new_size = winit::dpi::PhysicalSize::new(1, 1);
//! // Update gfx views, e.g. after a window resize
//! old_school_gfx_glutin_ext::resize_views(new_size, &mut color_view, &mut depth_view);
//! # Ok(()) }
//! ```

mod glutin_winit2;

use crate::glutin_winit2::GlWindow;
use gfx_core::{
    format::{ChannelType, DepthFormat, Format, RenderFormat},
    handle::{DepthStencilView, RawDepthStencilView, RawRenderTargetView, RenderTargetView},
    memory::Typed,
    texture,
};
use gfx_device_gl::Resources as R;
use glutin::{
    config::{ColorBufferType, ConfigTemplateBuilder},
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use raw_window_handle::HasRawWindowHandle;
use std::{error::Error, ffi::CString};

/// Returns a builder for initialising a winit window, glutin context & gfx views.
pub fn window_builder<T: 'static>(
    event_loop: &winit::event_loop::EventLoop<T>,
    winit: winit::window::WindowBuilder,
) -> Builder<'_, T> {
    Builder {
        event_loop,
        winit,
        surface_attrs: <_>::default(),
        ctx_attrs: <_>::default(),
        config_attrs: <_>::default(),
    }
}

/// Builder for initialising a winit window, glutin context & gfx views.
#[derive(Debug, Clone)]
pub struct Builder<'a, T: 'static> {
    event_loop: &'a winit::event_loop::EventLoop<T>,
    winit: winit::window::WindowBuilder,
    surface_attrs: Option<SurfaceAttributesBuilder<WindowSurface>>,
    ctx_attrs: ContextAttributesBuilder,
    config_attrs: ConfigTemplateBuilder,
}

impl<T> Builder<'_, T> {
    /// Configure surface attributes.
    ///
    /// If not called glutin default settings are used.
    pub fn surface_attributes(
        mut self,
        surface_attrs: SurfaceAttributesBuilder<WindowSurface>,
    ) -> Self {
        self.surface_attrs = Some(surface_attrs);
        self
    }

    /// Configure context attributes.
    ///
    /// If not called glutin default settings are used.
    pub fn context_attributes(mut self, ctx_attrs: ContextAttributesBuilder) -> Self {
        self.ctx_attrs = ctx_attrs;
        self
    }

    /// Configure [`ConfigTemplateBuilder`].
    pub fn config_template(mut self, conf: ConfigTemplateBuilder) -> Self {
        self.config_attrs = conf;
        self
    }

    /// Initialise a winit window, glutin context & gfx views.
    pub fn build<Color, Depth>(self) -> Result<Init<Color, Depth>, Box<dyn Error>>
    where
        Color: RenderFormat,
        Depth: DepthFormat,
    {
        self.build_raw(Color::get_format(), Depth::get_format())
            .map(|i| i.into_typed())
    }

    /// Initialise a winit window, glutin context & gfx views.
    pub fn build_raw(
        self,
        color_format: Format,
        depth_format: Format,
    ) -> Result<RawInit, Box<dyn Error>> {
        let Format(color_surface, color_channel) = color_format;
        let color_total_bits = color_surface.get_total_bits();
        let alpha_bits = color_surface.get_alpha_stencil_bits();
        let depth_total_bits = depth_format.0.get_total_bits();
        let stencil_bits = depth_format.0.get_alpha_stencil_bits();
        let srgb = color_channel == ChannelType::Srgb;
        let surface_attrs = self
            .surface_attrs
            .unwrap_or_else(|| SurfaceAttributesBuilder::new().with_srgb(srgb.then_some(true)));

        let config_attrs = self
            .config_attrs
            .with_alpha_size(alpha_bits)
            .with_depth_size(depth_total_bits - stencil_bits)
            .with_stencil_size(stencil_bits);

        let mut no_suitable_config = false;
        let (window, gl_config) = glutin_winit::DisplayBuilder::new()
            .with_window_builder(Some(self.winit))
            .build(self.event_loop, config_attrs, |configs| {
                let mut configs: Vec<_> = configs.collect();
                assert!(!configs.is_empty(), "no gl configs?");

                let best = configs
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| {
                        let color_bits = match c.color_buffer_type() {
                            None => 0,
                            Some(ColorBufferType::Luminance(s)) => s,
                            Some(ColorBufferType::Rgb {
                                r_size,
                                g_size,
                                b_size,
                            }) => r_size + g_size + b_size,
                        };

                        (!srgb || c.srgb_capable())
                            && color_bits == color_total_bits - alpha_bits
                            && c.alpha_size() == alpha_bits
                            && c.depth_size() == depth_total_bits - stencil_bits
                            && c.stencil_size() == stencil_bits
                    })
                    .max_by_key(|(_, c)| c.num_samples());
                match best {
                    Some((idx, _)) => configs.swap_remove(idx),
                    None => {
                        no_suitable_config = true;
                        configs.swap_remove(0)
                    }
                }
            })?;

        if no_suitable_config {
            return Err("no suitable gl config found, color+depth not supported?".into());
        }

        let window = window.unwrap(); // set in display builder
        let raw_window_handle = window.raw_window_handle();
        let gl_display = gl_config.display();

        let (gl_surface, gl_context) = {
            let ctx_attrs = self.ctx_attrs.build(Some(raw_window_handle));
            let surface_attrs = window.build_surface_attributes(surface_attrs);
            let surface = unsafe { gl_display.create_window_surface(&gl_config, &surface_attrs)? };
            let context = unsafe { gl_display.create_context(&gl_config, &ctx_attrs)? }
                .make_current(&surface)?;
            (surface, context)
        };

        let (device, factory) =
            gfx_device_gl::create(|s| gl_display.get_proc_address(&CString::new(s).unwrap()) as _);

        let window_size = window.inner_size();
        let tex_dimensions = (
            window_size.width as _,
            window_size.height as _,
            1,
            gl_config.num_samples().into(),
        );
        let (color_view, depth_view) =
            gfx_device_gl::create_main_targets_raw(tex_dimensions, color_surface, depth_format.0);

        Ok(RawInit {
            window,
            gl_config,
            gl_surface,
            gl_context,
            device,
            factory,
            color_view,
            depth_view,
        })
    }
}

/// Initialised winit, glutin & gfx state.
#[non_exhaustive]
pub struct InitState<ColorView, DepthView> {
    // winit
    pub window: winit::window::Window,
    // glutin
    pub gl_config: glutin::config::Config,
    pub gl_surface: glutin::surface::Surface<WindowSurface>,
    pub gl_context: glutin::context::PossiblyCurrentContext,
    // gfx
    pub device: gfx_device_gl::Device,
    pub factory: gfx_device_gl::Factory,
    pub color_view: ColorView,
    pub depth_view: DepthView,
}

/// "Raw" initialised winit, glutin & gfx state.
pub type RawInit = InitState<RawRenderTargetView<R>, RawDepthStencilView<R>>;
/// Initialised winit, glutin & gfx state.
pub type Init<Color, Depth> = InitState<RenderTargetView<R, Color>, DepthStencilView<R, Depth>>;

impl RawInit {
    fn into_typed<Color: RenderFormat, Depth: DepthFormat>(self) -> Init<Color, Depth> {
        Init {
            window: self.window,
            gl_config: self.gl_config,
            gl_surface: self.gl_surface,
            gl_context: self.gl_context,
            device: self.device,
            factory: self.factory,
            color_view: Typed::new(self.color_view),
            depth_view: Typed::new(self.depth_view),
        }
    }
}

/// Recreate and replace gfx views if the dimensions have changed.
pub fn resize_views<Color: RenderFormat, Depth: DepthFormat>(
    new_size: winit::dpi::PhysicalSize<u32>,
    color_view: &mut RenderTargetView<R, Color>,
    depth_view: &mut DepthStencilView<R, Depth>,
) {
    if let Some((cv, dv)) = resized_views(new_size, color_view, depth_view) {
        *color_view = cv;
        *depth_view = dv;
    }
}

/// Return new gfx views if the dimensions have changed.
#[must_use]
pub fn resized_views<Color: RenderFormat, Depth: DepthFormat>(
    new_size: winit::dpi::PhysicalSize<u32>,
    color_view: &RenderTargetView<R, Color>,
    depth_view: &DepthStencilView<R, Depth>,
) -> Option<(RenderTargetView<R, Color>, DepthStencilView<R, Depth>)> {
    let old_dimensions = color_view.get_dimensions();
    debug_assert_eq!(old_dimensions, depth_view.get_dimensions());

    let (cv, dv) = resized_views_raw(
        new_size,
        old_dimensions,
        Color::get_format(),
        Depth::get_format(),
    )?;

    Some((Typed::new(cv), Typed::new(dv)))
}

/// Return new gfx views if the dimensions have changed.
#[must_use]
pub fn resized_views_raw(
    new_size: winit::dpi::PhysicalSize<u32>,
    old_dimensions: texture::Dimensions,
    color_fmt: Format,
    ds_fmt: Format,
) -> Option<(RawRenderTargetView<R>, RawDepthStencilView<R>)> {
    let new_dimensions = (
        new_size.width as _,
        new_size.height as _,
        old_dimensions.2,
        old_dimensions.3,
    );
    if old_dimensions == new_dimensions {
        return None;
    }
    Some(gfx_device_gl::create_main_targets_raw(
        new_dimensions,
        color_fmt.0,
        ds_fmt.0,
    ))
}
