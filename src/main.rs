extern crate tcod;
extern crate nalgebra as na;
mod world;
mod renderer;

use tcod::input::{Key, KeyCode};
use tcod::console::Root;
use world::World;
use world::entities::Entity;
use renderer::{Renderer, RendererImpl};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use self::na::Point2;

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

        while !self.root.borrow().deref().window_closed() {
            let key: Key;

            match self.read_key() {
                Some(k) => key = k,
                None => break,
            }

            self.world.update(key);
            self.render();
        }
    }

    fn read_key(&mut self) -> Option<Key> {
        let keypress = self.root.borrow_mut().deref_mut().wait_for_keypress(true);

        match keypress.code
        {
            KeyCode::Escape => return None,
            _ => (),
        }

        Some(keypress)
    }

    fn render(&mut self) {
        {
            let player = self.world.get_player();
            let center: Point2<i64> = player.get_location();
            self.renderer.deref_mut().set_viewport_center(center);
        }
        self.world.render(&mut *self.renderer);
    }
}
