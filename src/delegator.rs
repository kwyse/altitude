use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

use entities::Velocity;

/// Controls an entity, such as through user input or through AI.
pub trait Delegator {
    /// The object that the delegator controls.
    type Delegate;

    /// Actions that will be taken to control the delegate.
    fn delegate(&mut self, delegate: &mut Self::Delegate);
}

/// Captures input to control the player entity.
pub struct PlayerInput<'pi> {
    events: &'pi mut EventPump
}

impl<'pi> PlayerInput<'pi> {
    pub fn new(events: &'pi mut EventPump) -> Self {
        Self {
            events: events,
        }
    }
}

impl<'pi> Delegator for PlayerInput<'pi> {
    type Delegate = Velocity;

    fn delegate(&mut self, delegate: &mut Velocity) {
        for event in self.events.poll_iter() {
            if let Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                println!("Escape!");
                panic!("Please abort me");
            }
        }

        let state = self.events.keyboard_state();
        if state.is_scancode_pressed(Scancode::D) {
            println!("Going down!");
            delegate.x = 0.0;
            delegate.y = 1.0;
        }
    }
}