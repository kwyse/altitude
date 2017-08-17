use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

use entities::Velocity;

pub trait Delegator {
    type Delegate;

    fn delegate(&mut self, delegate: &mut Self::Delegate);
}

pub struct UserInput<'ui> {
    events: &'ui mut EventPump
}

impl<'ui> UserInput<'ui> {
    pub fn new(events: &'ui mut EventPump) -> Self {
        Self {
            events: events,
        }
    }
}

impl<'ui> Delegator for UserInput<'ui> {
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