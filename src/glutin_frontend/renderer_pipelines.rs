use gfx::Factory;
use gfx_device_gl;

use glutin_frontend::tile_map::TileMapPipeline;
use glutin_frontend::scale::ScalePipeline;
use glutin_frontend::types::*;
use glutin_frontend::world_tile;
use glutin_frontend::overlay_tile;

use tile_desc::TileDesc;
use tile;

pub struct RendererPipelines {
    pub world: TileMapPipeline,
    pub overlay: TileMapPipeline,
    pub scale: ScalePipeline,
    pub description: TileDesc,
}

struct ViewPair(ShaderResourceView, RenderTargetView);

impl ViewPair {
    fn new(width: u32, height: u32, factory: &mut gfx_device_gl::Factory) -> Self {
        let (_, resource, target) = factory.create_render_target(width as u16, height as u16)
            .expect("Failed to create render target");

        ViewPair(resource, target)
    }
}

impl RendererPipelines {
    pub fn new(width_tiles: u32,
               height_tiles: u32,
               rtv: RenderTargetView,
               factory: &mut gfx_device_gl::Factory,
               encoder: &mut Encoder) -> Self {

        let (tile_img, tile_desc) = tile::read_tiles();

        let buf_width_px = width_tiles * tile_desc.tile_size;
        let buf_height_px = height_tiles * tile_desc.tile_size;

        let ViewPair(world_resource, world_target) = ViewPair::new(buf_width_px, buf_height_px, factory);
        let ViewPair(overlay_resource, overlay_target) = ViewPair::new(buf_width_px, buf_height_px, factory);

        let mut world_pipeline = TileMapPipeline::new(width_tiles, height_tiles, (width_tiles * height_tiles) as usize,
                                                      include_bytes!("shaders/shdr_330.vert"),
                                                      include_bytes!("shaders/shdr_world_330.frag"),
                                                      world_tile::shader_template_info(),
                                                      &tile_img,
                                                      &tile_desc,
                                                      world_target,
                                                      factory,
                                                      encoder);

        world_tile::init_tile_map_data(&mut world_pipeline.buffer);

        let overlay_pipeline = TileMapPipeline::new(height_tiles, height_tiles, (height_tiles * height_tiles) as usize,
                                                    include_bytes!("shaders/shdr_330.vert"),
                                                    include_bytes!("shaders/shdr_overlay_330.frag"),
                                                    overlay_tile::shader_template_info(),
                                                    &tile_img,
                                                    &tile_desc,
                                                    overlay_target,
                                                    factory,
                                                    encoder);


        let scale_pipeline = ScalePipeline::new(include_bytes!("shaders/shdr_scale_330.vert"),
                                                include_bytes!("shaders/shdr_scale_330.frag"),
                                                buf_width_px,
                                                buf_height_px,
                                                world_resource,
                                                overlay_resource,
                                                rtv,
                                                factory,
                                                encoder);
        RendererPipelines {
            world: world_pipeline,
            overlay: overlay_pipeline,
            scale: scale_pipeline,
            description: tile_desc,
        }
    }
}
