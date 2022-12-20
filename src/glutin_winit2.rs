//! glutin-winit helpers (proposed <https://github.com/rust-windowing/glutin/pull/1545>)
use glutin::surface::{SurfaceAttributes, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use winit::window::Window;

/// [`Window`] extensions for working with [`glutin`] surfaces.
pub(crate) trait GlWindow {
    /// Build the surface attributes suitable to create a window surface.
    ///
    /// Panics if either window inner dimension is zero.
    fn build_surface_attributes(
        &self,
        builder: SurfaceAttributesBuilder<WindowSurface>,
    ) -> SurfaceAttributes<WindowSurface>;
}

impl GlWindow for Window {
    fn build_surface_attributes(
        &self,
        builder: SurfaceAttributesBuilder<WindowSurface>,
    ) -> SurfaceAttributes<WindowSurface> {
        let (w, h) = self
            .inner_size()
            .non_zero()
            .expect("invalid zero inner size");
        builder.build(self.raw_window_handle(), w, h)
    }
}

/// [`winit::dpi::PhysicalSize<u32>`] non-zero extensions.
pub(crate) trait NonZeroU32PhysicalSize {
    /// Converts to non-zero `(width, height)`.
    fn non_zero(self) -> Option<(NonZeroU32, NonZeroU32)>;
}
impl NonZeroU32PhysicalSize for winit::dpi::PhysicalSize<u32> {
    fn non_zero(self) -> Option<(NonZeroU32, NonZeroU32)> {
        let w = NonZeroU32::new(self.width)?;
        let h = NonZeroU32::new(self.height)?;
        Some((w, h))
    }
}
