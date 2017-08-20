use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

use delegator::{EnemyHeuristic, PlayerInput};
use entities::{Entity, Size};
use graphics::Sprite;
use physics::Physics;
use resources::Resources;

pub trait View {
    type Target;

    fn process_input(&mut self, events: &Vec<Event>) -> Action;
    fn update(&mut self, elapsed: &Duration);
    fn render(&mut self, target: &mut Self::Target, elapsed: &Duration);
}

#[derive(PartialEq)]
pub enum Action {
    Continue,
    Quit,
}

pub struct GameView<'e> {
    player: Entity<PlayerInput, Physics, Sprite<'e>>,
    enemies: Vec<Entity<EnemyHeuristic, Physics, Sprite<'e>>>,
}

impl<'e> GameView<'e> {
    pub fn new(resources: &'e Resources) -> Self {
        let player_input = PlayerInput::new();
        let player_physics = Physics;
        let player_sprite = Sprite::new(&resources.textures.player, Size { width: 32, height: 32 });
        let player = Entity::new(player_input, player_physics, player_sprite);

        let enemy_heuristic = EnemyHeuristic;
        let enemy_physics = Physics;
        let enemy_sprite = Sprite::new(&resources.textures.enemy, Size { width: 32, height: 32 });
        let enemy = Entity::new(enemy_heuristic, enemy_physics, enemy_sprite);

        Self {
            player: player,
            enemies: vec![enemy],
        }
    }
}

impl<'e> View for GameView<'e> {
    type Target = Canvas<Window>;

    fn process_input(&mut self, events: &Vec<Event>) -> Action {
        for event in events.iter() {
            if let &Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                return Action::Quit;
            }
        }

        self.player.delegate(events);
        Action::Continue
    }

    fn update(&mut self, elapsed: &Duration) {
        self.player.update(elapsed);

        for enemy in &mut self.enemies {
            enemy.delegate(&self.player.position());
            enemy.update(elapsed);
        }
    }

    fn render(&mut self, target: &mut Self::Target, _elapsed: &Duration) {
        self.player.render(target);

        for enemy in &mut self.enemies {
            enemy.render(target);
        }
    }
}