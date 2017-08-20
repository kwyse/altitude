use std::time::Duration;

use entities::{Position, Velocity};

pub trait Movable {
    type Mutator;
    type Mutable;

    fn resolve(&mut self, mutator: &Self::Mutator, mutable: &mut Self::Mutable, elapsed: &Duration);
}

pub struct Physics;

impl Movable for Physics {
    type Mutator = Velocity;
    type Mutable = Position;

    fn resolve(&mut self, mutator: &Self::Mutator, mutable: &mut Self::Mutable, elapsed: &Duration) {
        mutable.x += mutator.x * elapsed.subsec_nanos() as f64 * 0.0000002;
        mutable.y += mutator.y * elapsed.subsec_nanos() as f64 * 0.0000002;
    }
}