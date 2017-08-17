use sdl2::render::Canvas;
use sdl2::video::Window;

use delegator::Delegator;
use graphics::Renderable;

pub struct Entity<'e, D: 'e, R: 'e> {
    delegator: &'e mut D,
    renderable: &'e mut R,
    velocity: Velocity,
    position: Position
}

impl<'e, D, R> Entity<'e, D, R>
    where D: Delegator<Delegate = Velocity>, R: Renderable<Target = Canvas<Window>>
{
    pub fn new(delegator: &'e mut D, renderable: &'e mut R) -> Self {
        Self {
            delegator: delegator,
            renderable: renderable,
            velocity: Velocity { x: 0.0, y: 0.0 },
            position: Position { x: 0.0, y: 0.0 },
        }
    }

    pub fn delegate(&mut self) {
        self.delegator.delegate(&mut self.velocity);
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