pub mod entities;

use nalgebra::Point2;
use tcod::input::Key;
use tcod::Console;

use world::entities::*;
use renderer::Renderer;

pub struct World {
    player: Player,
    entities: Vec<Box<Entity>>,
}

impl World {
    pub fn new() -> World {
        World {
            player: Player::new(),
            entities: vec![Box::new(Rock::new(Point2::new(10, 5)))],
        }
    }

    pub fn update(&mut self, key: Key) {
        self.player.set_last_keypress(key);
        self.player.update();

        for entity in &mut self.entities {
            entity.update();
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        renderer.clear();

        self.player.render(renderer);

        for entity in &self.entities {
            entity.render(renderer);
        }

        renderer.flush();
    }

    pub fn get_player(&self) -> &Player {
       &self.player
    }
}
