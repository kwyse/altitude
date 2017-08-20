use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use entities::Velocity;

/// Controls an entity, such as through user input or through AI.
pub trait Delegator {
    /// The object that the delegator controls.
    type Delegate;
    /// The object controlling the delegate.
    type Delegator;

    /// Actions that will be taken to control the delegate.
    fn delegate(&mut self, delegator: &Self::Delegator, delegate: &mut Self::Delegate);
}

/// Captures input to control the player entity.
pub struct PlayerInput;

impl<'ui> Delegator for PlayerInput {
    type Delegate = Velocity;
    type Delegator = Vec<Event>;

    fn delegate(&mut self, delegator: &Self::Delegator, _delegate: &mut Self::Delegate) {
        for event in delegator.iter() {
            if let &Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                println!("Escape!");
                panic!("Please abort me");
            }
        }
    }
}