extern crate sdl2;

use std::time::{Duration, Instant};

pub mod delegator;
pub mod entities;
pub mod graphics;
pub mod physics;
pub mod resources;
pub mod views;

use resources::Resources;
use views::{Action, GameView, View};

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("Altitude", 640, 480).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = &canvas.texture_creator();
    let resources = Resources::new(texture_creator);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let frame_duration = Duration::from_millis(1_000 / 60);
    let mut previous = Instant::now();
    let mut lag = Duration::new(0, 0);

    let mut current_view = GameView::new(&resources);

    'running: loop {
        let current = Instant::now();
        let elapsed = current - previous;
        previous = current;
        lag += elapsed;

        let events = event_pump.poll_iter().collect();

        let view_action = current_view.process_input(&events);
        if view_action == Action::Quit {
            break 'running;
        }

        while lag > frame_duration {
            lag -= frame_duration;

            current_view.update(&elapsed);
        }

        canvas.clear();
        current_view.render(&mut canvas, &elapsed);
        canvas.present();
    }
}