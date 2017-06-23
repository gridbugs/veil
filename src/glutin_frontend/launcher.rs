use gfx;
use gfx::Device;
use gfx::Factory;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin;
use image;

pub type ColourFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const WIDTH_TILES: u32 = 20;
const HEIGHT_TILES: u32 = 20;
const TILE_SIZE: u32 = 32;

const WIDTH_PX: u32 = WIDTH_TILES * TILE_SIZE;
const HEIGHT_PX: u32 = HEIGHT_TILES * TILE_SIZE;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        tex_pos: [f32; 2] = "a_TexPos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        out: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

pub fn launch() {

    let builder = glutin::WindowBuilder::new()
        .with_dimensions(WIDTH_PX, HEIGHT_PX)
        .with_title("Veil".to_string());

    let events_loop = glutin::EventsLoop::new();

    let (window, mut device, mut factory, colour_view, _main_depth) =
        gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, &events_loop);

    let img = image::open("resources/tiles.png").expect("failed to open image").to_rgba();
    let (img_width, img_height) = img.dimensions();
    let tex_kind = gfx::texture::Kind::D2(img_width as u16, img_height as u16, gfx::texture::AaMode::Single);
    let (_, texture) = factory.create_texture_immutable_u8::<ColourFormat>(tex_kind, &[&img])
        .expect("Failed to create texture");
    let sampler = factory.create_sampler_linear();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/shdr_150.vert"),
        include_bytes!("shaders/shdr_150.frag"),
        pipe::new()
    ).expect("Failed to create pipeline");

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    const SQUARE: [Vertex; 4] = [
        Vertex { pos: [-1.0, 1.0], tex_pos: [0.0, 0.0] },
        Vertex { pos: [1.0, 1.0], tex_pos: [1.0, 0.0] },
        Vertex { pos: [1.0, -1.0], tex_pos: [1.0, 1.0] },
        Vertex { pos: [-1.0, -1.0], tex_pos: [0.0, 1.0] },
    ];

    const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

    let index_data: &[u16] = &[0, 1, 3, 2, 3, 1];
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQUARE, index_data);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        tex: (texture, sampler),
        out: colour_view,
    };

    'main: loop {
        let mut running = true;
        events_loop.poll_events(|e| {
            let event = if let glutin::Event::WindowEvent { event, .. } = e {
                event
            } else {
                return;
            };
            match event {
                glutin::WindowEvent::Closed => running = false,
                _ => {}
            }
        });

        if !running {
            break;
        }

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().expect("Failed to swap buffers");
        device.cleanup();
    }
}
