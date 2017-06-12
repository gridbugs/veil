use sdl2::VideoSubsystem;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::GLProfile;

pub struct RendererEnv {
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl RendererEnv {
    pub fn new(width_px: u32, height_px: u32, video: &VideoSubsystem) -> Self {

        let gl_attr = video.gl_attr();

        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_double_buffer(true);
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        let canvas = video.window("Veil", width_px, height_px)
            .opengl()
            .position_centered()
            .build()
            .expect("Failed to create window")
            .into_canvas()
            .build()
            .expect("Failed to create canvas");

        let texture_creator = canvas.texture_creator();

        RendererEnv {
            canvas: canvas,
            texture_creator: texture_creator,
        }
    }
}
