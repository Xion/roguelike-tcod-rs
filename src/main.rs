extern crate tcod;
extern crate nalgebra;
mod world;
mod renderer;

use std::rc::Rc;
use std::cell::RefCell;

use nalgebra::Point2;
use tcod::console::Root;
use tcod::input::{Key, KeyCode};

use world::World;
use world::entities::Entity;
use renderer::{Renderer, RendererImpl};

fn main() {
    let root = Root::initializer()
        .fullscreen(false)
        .title("Roguelike")
        .size(80, 50)
        .init();

    Game::new(root).start();
}

struct Game {
    root: Rc<RefCell<Root>>,
    world: World,
    renderer: Box<Renderer>,
}

impl Game {
    fn new(root: Root) -> Game {
        let root = Rc::new(RefCell::new(root));
        Game {
            root: root.clone(),
            world: World::new(),
            renderer: Box::new(RendererImpl::new(root.clone())),
        }
    }

    fn start(&mut self) {
        self.render();

        while !self.root.borrow().window_closed() {
            let key: Key = match self.read_key() {
                Some(k) => k,
                None => break,
            };

            self.world.update(key);
            self.render();
        }
    }

    fn read_key(&mut self) -> Option<Key> {
        let keypress = self.root.borrow_mut().wait_for_keypress(true);
        match keypress.code {
            KeyCode::Escape => None,
            _ => Some(keypress),
        }
    }

    fn render(&mut self) {
        {
            let player = self.world.get_player();
            let center: Point2<i64> = player.get_location();
            self.renderer.set_viewport_center(center);
        }
        self.world.render(&mut *self.renderer);
    }
}
