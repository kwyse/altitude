extern crate sdl2;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::video::Window;
use std::time::{Duration, Instant};

pub struct Velocity {
    x: f64,
    y: f64,
}

pub struct Position {
    x: f64,
    y: f64,
}

pub struct Size {
    width: u32,
    height: u32,
}

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

pub trait Renderable {
    type Target;

    fn render(&mut self, target: &mut Self::Target, position: &Position);
}

pub struct Sprite<'t> {
    texture: &'t Texture<'t>,
    size: Size,
}

impl<'t> Sprite<'t> {
    pub fn new(texture: &'t Texture, size: Size) -> Self {
        Self {
            texture: texture,
            size: size,
        }
    }
}

impl<'t> Renderable for Sprite<'t> {
    type Target = Canvas<Window>;

    fn render(&mut self, target: &mut Self::Target, position: &Position) {
        let source = Rect::new(0, 0, self.size.width, self.size.height);

        let pos_x = position.x as i32;
        let pos_y = position.y as i32;
        let destination = Rect::new(pos_x, pos_y, self.size.width, self.size.height);

        target.copy(self.texture, Some(source), Some(destination)).ok();
    }
}

pub struct Resources<'r> {
    pub textures: Textures<'r>,
}

impl<'r> Resources<'r> {
    pub fn new<T>(texture_loader: &'r TextureCreator<T>) -> Self {
        Self {
            textures: Textures::new(texture_loader),
        }
    }
}

pub struct Textures<'t> {
    pub player: Texture<'t>,
}

impl<'t> Textures<'t> {
    pub fn new<T>(loader: &'t TextureCreator<T>) -> Self {
        Self {
            player: loader.load_texture("assets/sprites/player.png").unwrap(),
        }
    }
}

pub struct Entity<'e, D: 'e, R: 'e> {
    delegator: &'e mut D,
    renderable: &'e mut R,
    velocity: Velocity,
    position: Position
}

impl<'e, D, R> Entity<'e, D, R>
    where D: Delegator<Delegate = Velocity>, R: Renderable<Target = Canvas<Window>>
{
    pub fn new(delegator: &'e mut D, renderable: &'e mut R) -> Self {
        Self {
            delegator: delegator,
            renderable: renderable,
            velocity: Velocity { x: 0.0, y: 0.0 },
            position: Position { x: 0.0, y: 0.0 },
        }
    }

    pub fn delegate(&mut self) {
        self.delegator.delegate(&mut self.velocity);
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.renderable.render(canvas, &self.position);
    }
}

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let mut events = sdl_context.event_pump().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("Altitude", 640, 480).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = &canvas.texture_creator();
    let resources = Resources::new(texture_creator);

    let mut input = UserInput::new(&mut events);
    let mut sprite = Sprite::new(&resources.textures.player, Size { width: 32, height: 32 });

    let mut entity = Entity::new(&mut input, &mut sprite);

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