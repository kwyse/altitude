use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

use delegator::Delegator;
use graphics::Renderable;
use physics::Movable;

/// A game object.
///
/// Composed of controlling, physics, and rendering components.
pub struct Entity<D, M, R> {
    delegator: Box<D>,
    movable: Box<M>,
    renderable: Box<R>,
    velocity: Velocity,
    position: Position,
}

impl<D, M, R> Entity<D, M, R>
where D: Delegator<Delegate = Velocity>,
      M: Movable<Mutator = Velocity, Mutable = Position>,
      R: Renderable<Target = Canvas<Window>>,
{
    pub fn new(delegator: D, movable: M, renderable: R) -> Self {
        Self {
            delegator: Box::new(delegator),
            movable: Box::new(movable),
            renderable: Box::new(renderable),
            velocity: Velocity { x: 0.0, y: 0.0 },
            position: Position { x: 0.0, y: 0.0 },
        }
    }

    pub fn delegate(&mut self, delegator: &D::Delegator) {
        self.delegator.delegate(delegator, &mut self.velocity);
    }

    pub fn update(&mut self, elapsed: &Duration) {
        self.movable.resolve(&self.velocity, &mut self.position, elapsed);
    }

    pub fn render(&mut self, target: &mut R::Target) {
        self.renderable.render(target, &self.position);
    }

    pub fn position(&self) -> &Position {
        &self.position
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