use gfx;
use gfx_device_gl;

use glutin_frontend::formats::ColourFormat;

pub type Resources = gfx_device_gl::Resources;
pub type Slice = gfx::Slice<Resources>;
pub type RenderTargetView = gfx::handle::RenderTargetView<Resources, ColourFormat>;
pub type Encoder = gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>;
pub type ShaderResourceView = gfx::handle::ShaderResourceView<Resources, [f32; 4]>;
