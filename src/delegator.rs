use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use std::marker::PhantomData;

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
pub struct PlayerInput<T> {
    _marker: PhantomData<T>
}

impl<T> PlayerInput<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'ui> Delegator for PlayerInput<UserInput<'ui>> {
    type Delegate = Velocity;
    type Delegator = UserInput<'ui>;

    fn delegate(&mut self, delegator: &Self::Delegator, delegate: &mut Self::Delegate) {
        for event in delegator.events.iter() {
            if let &Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                println!("Escape!");
                panic!("Please abort me");
            }
        }

        let state = &delegator.keyboard;
        if state.is_scancode_pressed(Scancode::S) {
            println!("Going down!");
            delegate.x = 0.0;
            delegate.y = 1.0;
        }
    }
}

pub struct UserInput<'ui> {
    events: Vec<Event>,
    keyboard: KeyboardState<'ui>,
}

impl<'ui> UserInput<'ui> {
    pub fn new(events: Vec<Event>, keyboard_state: KeyboardState<'ui>) -> Self {
        Self {
            events: events,
            keyboard: keyboard_state,
        }
    }
}