extern crate sdl2;

use std::time::{Duration, Instant};

pub mod delegator;
pub mod entities;
pub mod graphics;
pub mod resources;

use delegator::PlayerInput;
use entities::{Entity, Size};
use graphics::Sprite;
use resources::Resources;

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let mut events = sdl_context.event_pump().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("Altitude", 640, 480).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = &canvas.texture_creator();
    let resources = Resources::new(texture_creator);

    let mut player_input = PlayerInput::new(&mut events);
    let mut sprite = Sprite::new(&resources.textures.player, Size { width: 32, height: 32 });

    let mut entity = Entity::new(&mut player_input, &mut sprite);

    let frame_duration = Duration::from_millis(1_000 / 60);
    let mut previous = Instant::now();
    let mut lag = Duration::new(0, 0);

    'running: loop {
        let current = Instant::now();
        let elapsed = current - previous;
        previous = current;
        lag += elapsed;

        entity.delegate();

        while lag > frame_duration {
            lag -= frame_duration;
        }

        canvas.clear();
        entity.render(&mut canvas);
        canvas.present();
    }
}