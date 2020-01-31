old_school_gfx_glutin_ext
[![crates.io](https://img.shields.io/crates/v/old_school_gfx_glutin_ext.svg)](https://crates.io/crates/old_school_gfx_glutin_ext)
[![Documentation](https://docs.rs/old_school_gfx_glutin_ext/badge.svg)](https://docs.rs/old_school_gfx_glutin_ext)
=========================

Extensions for [glutin](https://crates.io/crates/glutin) to initialize & update old school
[gfx](https://crates.io/crates/gfx). _An alternative to gfx_window_glutin_.

```rust
use old_school_gfx_glutin_ext::*;

type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

// Initialize
let (window_ctx, mut device, mut factory, mut main_color, mut main_depth) =
    glutin::ContextBuilder::new()
        .with_gfx_color_depth::<ColorFormat, DepthFormat>()
        .build_windowed(window_config, &event_loop)?
        .init_gfx::<ColorFormat, DepthFormat>();

// Update, ie after a resize
window_ctx.update_gfx(&mut main_color, &mut main_depth);
```

## Example
`cargo run --example triangle`

## Minimum supported rust compiler
All crates maintained with [latest stable rust](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).
