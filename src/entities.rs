use sdl2::render::Canvas;
use sdl2::video::Window;

use delegator::Delegator;
use graphics::Renderable;

/// A game object.
///
/// Composed of a controlling component and a rendering component.
pub struct Entity<D, R> {
    delegator: Box<D>,
    renderable: Box<R>,
    velocity: Velocity,
    position: Position,
}

impl<D, R> Entity<D, R>
where D: Delegator<Delegate = Velocity>,
      R: Renderable<Target = Canvas<Window>>,
{
    pub fn new(delegator: D, renderable: R) -> Self {
        Self {
            delegator: Box::new(delegator),
            renderable: Box::new(renderable),
            velocity: Velocity { x: 0.0, y: 0.0 },
            position: Position { x: 0.0, y: 0.0 },
        }
    }

    pub fn delegate(&mut self, delegator: &D::Delegator) {
        self.delegator.delegate(delegator, &mut self.velocity);
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.renderable.render(canvas, &self.position);
    }
}

pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}