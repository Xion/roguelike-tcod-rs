use tcod::input::{Key, KeyCode};
use nalgebra::Point2;

use renderer::Renderer;

pub trait Entity {
    fn get_location(&self) -> Point2<i64>;
    fn update(&mut self);
    fn render(&self, renderer: &mut Renderer);
}

pub struct Player {
    last_keypress: Key,
    location: Point2<i64>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            last_keypress: Key::default(),
            location: Point2::new(0, 0),
        }
    }

    pub fn set_location(&mut self, location: &Point2<i64>) {
        self.location.x = location.x;
        self.location.y = location.y;
    }

    pub fn set_last_keypress(&mut self, last_keypress: Key) {
        self.last_keypress = last_keypress;
    }
}

impl Entity for Player {
    fn get_location(&self) -> Point2<i64> {
        self.location
    }

    fn update(&mut self) {
        match self.last_keypress.code {
            KeyCode::Left => self.location.x -= 1,
            KeyCode::Down => self.location.y += 1,
            KeyCode::Up => self.location.y -= 1,
            KeyCode::Right => self.location.x += 1,
            _ => {}
        }
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_char(self.location, '@');
    }
}

pub struct Rock {
    location: Point2<i64>,
}

impl Rock {
    pub fn new(location: Point2<i64>) -> Rock {
        Rock {
            location: location,
        }
    }
}

impl Entity for Rock {
    fn get_location(&self) -> Point2<i64> {
        self.location
    }

    fn update(&mut self) {

    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_char(self.location, 'o');
    }
}
