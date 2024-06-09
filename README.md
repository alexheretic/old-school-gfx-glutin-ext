old_school_gfx_glutin_ext
[![crates.io](https://img.shields.io/crates/v/old_school_gfx_glutin_ext.svg)](https://crates.io/crates/old_school_gfx_glutin_ext)
[![Documentation](https://docs.rs/old_school_gfx_glutin_ext/badge.svg)](https://docs.rs/old_school_gfx_glutin_ext)
=========================

Initialise & update old school [gfx](https://crates.io/crates/gfx) with [glutin](https://crates.io/crates/glutin) + [winit](https://crates.io/crates/winit).

```rust
type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

let event_loop = winit::event_loop::EventLoop::new()?;
let window_attrs = winit::window::Window::default_attributes();

// Initialise winit window, glutin context & gfx views
let old_school_gfx_glutin_ext::Init {
    // winit window
    window,
    // glutin bits
    gl_config,
    gl_surface,
    gl_context,
    // gfx bits
    mut device,
    mut factory,
    mut color_view,
    mut depth_view,
    ..
} = old_school_gfx_glutin_ext::window_builder(&event_loop, window_attrs)
    .build::<ColorFormat, DepthFormat>()?;

// Update gfx views, e.g. after a window resize
old_school_gfx_glutin_ext::resize_views(new_size, &mut color_view, &mut depth_view);
```

## Example
`cargo run --example triangle`

## Minimum supported rust compiler
All crates maintained with [latest stable rust](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).
