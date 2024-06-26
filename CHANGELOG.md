# v0.34.0
* Require `winit::event_loop::ActiveEventLoop` for window building supporting `winit::application::ApplicationHandler`.
  Use previous version for building using `EventLoop`.

# v0.33.0
* Update _winit_ to `0.30`, _glutin-winit_ to `0.5`, _glutin_ to `0.32`, _raw-window-handle_ to `0.6`.

# v0.32.1
* Allow incorrect OS config depth_size = expected depth_size+stencil_size.
  This works around an issue with macos software rendering.

# v0.32.0
* Update _winit_ to `0.29`, _glutin-winit_ to `0.4`, _glutin_ to `0.31`.

# v0.31.0
* Update _winit_ to `0.28`, _glutin-winit_ to `0.3`.

# v0.30.2
* Update _glutin-winit_ & reduce code.

# v0.30.1
* Add `Builder::number_of_samples` method to configure multisample buffer sample count preference. Default **0**.

# v0.30.0
* Reworked API to support _glutin_ `0.30`. See new docs & examples.
* Crate version will no longer follow _glutin_ as now _winit_ releases may separately cause breaking changes.

# v0.29.2
* Fix docs.rs build.

# v0.29.1
* Remove all feature requirements from _glutin_ to allow disabling _glutin_ features.

# v0.29.0
* Update _glutin_ to `0.29`.

# v0.28.0
* Update _glutin_ to `0.28`.

# v0.27.0
* Update _glutin_ to `0.27`.

# v0.26.0
* Update _glutin_ to `0.26`.

# v0.25.0
* Update _glutin_ to `0.25`.

# v0.24.0
* Update _glutin_ to `0.24`.

# v0.23.0
* Update _glutin_ to `0.23`.

# v0.22.1
* Add `ContextBuilderExt::with_gfx_color_raw`, `ContextBuilderExt::with_gfx_depth_raw` methods.

# v0.22.0
Initial release supporting _glutin_ `0.22`.
