use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

use delegator::PlayerInput;
use entities::{Entity, Size};
use graphics::Sprite;
use resources::Resources;

pub trait View {
    type Target;

    fn process_input(&mut self, events: &Vec<Event>) -> Action;
    fn update(&self, elapsed: &Duration);
    fn render(&mut self, target: &mut Self::Target, elapsed: &Duration);
}

#[derive(PartialEq)]
pub enum Action {
    Continue,
    Quit,
}

pub struct GameView<'e> {
    player: Entity<PlayerInput, Sprite<'e>>,
}

impl<'e> GameView<'e> {
    pub fn new(resources: &'e Resources) -> Self {
        let player_input = PlayerInput::new();
        let player_sprite = Sprite::new(&resources.textures.player, Size { width: 32, height: 32 });
        let player = Entity::new(player_input, player_sprite);

        Self {
            player: player,
        }
    }
}

impl<'e> View for GameView<'e> {
    type Target = Canvas<Window>;

    fn process_input(&mut self, events: &Vec<Event>) -> Action {
        self.player.delegate(events);
        Action::Continue
    }

    fn update(&self, _elapsed: &Duration) {
    }

    fn render(&mut self, target: &mut Self::Target, _elapsed: &Duration) {
        self.player.render(target);
    }
}