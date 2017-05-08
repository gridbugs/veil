use std::path::Path;
use std::cmp;
use std::slice;
use std::mem;
use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;
use sdl2::surface::Surface;
use sdl2::image::LoadSurface;

pub struct GameTextures<'a> {
    pub colour: Texture<'a>,
    pub greyscale: Texture<'a>,
}

impl<'a> GameTextures<'a> {
    pub fn new<P: AsRef<Path>>(tile_path: P, texture_creator: &'a TextureCreator<WindowContext>) -> Self {

        let colour = texture_creator.load_texture(&tile_path)
            .expect("Failed to create texture");

        let greyscale = create_greyscale_texture(&tile_path, texture_creator);

        GameTextures {
            colour: colour,
            greyscale: greyscale,
        }
    }
}

fn create_greyscale_texture<'a, P: AsRef<Path>>(tile_path: P, texture_creator: &'a TextureCreator<WindowContext>) -> Texture<'a> {

    let tile_surface = Surface::from_file(tile_path)
        .expect("Failed to load tile texture");

    let size = (tile_surface.width() * tile_surface.height()) as usize;

    let pixels = unsafe {
        let pixels_ptr = (&mut *tile_surface.raw()).pixels as *mut u32;
        slice::from_raw_parts_mut(pixels_ptr, size)
    };

    for pixel in pixels.iter_mut() {

        const R: usize = 0;
        const G: usize = 1;
        const B: usize = 2;

        let mut arr = unsafe { mem::transmute::<u32, [u8; 4]>(*pixel) };
        let max = cmp::max(arr[R], cmp::max(arr[G], arr[B])) as u32;
        let darkened = ((max * 1) / 3) as u8;

        arr[R] = darkened;
        arr[G] = darkened;
        arr[B] = darkened;

        *pixel = unsafe { mem::transmute::<[u8; 4], u32>(arr) };
    }

    texture_creator.create_texture_from_surface(&tile_surface)
        .expect("Failed to create greyscale texture")
}
