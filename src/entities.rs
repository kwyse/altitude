use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

use delegator::Delegator;
use graphics::Renderable;
use physics::Physics;

/// A game object.
///
/// Composed of controlling, physics, and rendering components.
pub struct Entity<D, P, R> {
    delegator: Box<D>,
    physics: Box<P>,
    renderable: Box<R>,
    velocity: Velocity,
    position: Position,
}

impl<D, P, R> Entity<D, P, R>
where D: Delegator<Delegate = Velocity>,
      P: Physics<Mutator = Velocity, Mutable = Position>,
      R: Renderable<Target = Canvas<Window>>,
{
    pub fn new(delegator: D, physics: P, renderable: R) -> Self {
        Self {
            delegator: Box::new(delegator),
            physics: Box::new(physics),
            renderable: Box::new(renderable),
            velocity: Velocity { x: 0.0, y: 0.0 },
            position: Position { x: 0.0, y: 0.0 },
        }
    }

    pub fn delegate(&mut self, delegator: &D::Delegator) {
        self.delegator.delegate(delegator, &mut self.velocity);
    }

    pub fn update(&mut self, elapsed: &Duration) {
        self.physics.resolve(&self.velocity, &mut self.position, elapsed);
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