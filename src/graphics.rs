use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use entities::{Position, Size};

/// Renders an entity to a render target.
pub trait Renderable {
    /// The target to render to.
    type Target;

    /// Steps to render the entity to the target.
    fn render(&mut self, target: &mut Self::Target, position: &Position);
}

/// A graphic rendered to screen that holds a reference to a texture.
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