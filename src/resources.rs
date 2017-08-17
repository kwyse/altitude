use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, Texture};

pub struct Resources<'r> {
    pub textures: Textures<'r>,
}

impl<'r> Resources<'r> {
    pub fn new<T>(texture_loader: &'r TextureCreator<T>) -> Self {
        Self {
            textures: Textures::new(texture_loader),
        }
    }
}

pub struct Textures<'t> {
    pub player: Texture<'t>,
}

impl<'t> Textures<'t> {
    pub fn new<T>(loader: &'t TextureCreator<T>) -> Self {
        Self {
            player: loader.load_texture("assets/textures/player.png").unwrap(),
        }
    }
}