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
pub struct PlayerInput {
    movement: MovementKeys,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            movement: MovementKeys {
                up: false,
                down: false,
                left: false,
                right: false,
            },
        }
    }
}
impl<'ui> Delegator for PlayerInput {
    type Delegate = Velocity;
    type Delegator = Vec<Event>;

    fn delegate(&mut self, delegator: &Self::Delegator, delegate: &mut Self::Delegate) {
        for event in delegator.iter() {
            match event {
                &Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::W => self.movement.up = true,
                        Keycode::S => self.movement.down = true,
                        Keycode::A => self.movement.left = true,
                        Keycode::D => self.movement.right = true,
                        Keycode::Escape => {
                            println!("Escape!");
                            panic!("Please abort me");
                        }
                        _ => { },
                    }
                },
                &Event::KeyUp { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::W => self.movement.up = false,
                        Keycode::S => self.movement.down = false,
                        Keycode::A => self.movement.left = false,
                        Keycode::D => self.movement.right = false,
                        _ => { },
                    }
                },
                _ => { },
            }
        }

        let mut vel_y = delegate.y;
        if self.movement.up { vel_y = -1.0_f64.max(vel_y - 1.0); } else { vel_y = 0.0_f64.max(vel_y); }
        if self.movement.down { vel_y = 1.0_f64.min(vel_y + 1.0); } else { vel_y = 0.0_f64.min(vel_y); }

        let mut vel_x = delegate.x;
        if self.movement.left { vel_x = -1.0_f64.max(vel_x - 1.0); } else { vel_x = 0.0_f64.max(vel_x); }
        if self.movement.right { vel_x = 1.0_f64.min(vel_x + 1.0); } else { vel_x = 0.0_f64.min(vel_x); }

        delegate.x = vel_x;
        delegate.y = vel_y;

        println!("({}, {})", delegate.x, delegate.y);
    }
}

struct MovementKeys {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}