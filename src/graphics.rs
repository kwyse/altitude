use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use entities::{Position, Size};

pub trait Renderable {
    type Target;

    fn render(&mut self, target: &mut Self::Target, position: &Position);
}

pub struct Sprite<'t> {
    texture: &'t Texture<'t>,
    size: Size,
}

impl<'t> Sprite<'t> {
    pub fn new(texture: &'t Texture, size: Size) -> Self {
        Self {
            texture: texture,
            size: size,
        }
    }
}

impl<'t> Renderable for Sprite<'t> {
    type Target = Canvas<Window>;

    fn render(&mut self, target: &mut Self::Target, position: &Position) {
        let source = Rect::new(0, 0, self.size.width, self.size.height);

        let pos_x = position.x as i32;
        let pos_y = position.y as i32;
        let destination = Rect::new(pos_x, pos_y, self.size.width, self.size.height);

        target.copy(self.texture, Some(source), Some(destination)).ok();
    }
}