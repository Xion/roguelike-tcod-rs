extern crate nalgebra as na;

use tcod::console::{Root, BackgroundFlag};
use tcod::Console;
use self::na::Point2;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

pub trait Renderer {
    fn clear(&mut self);
    fn flush(&mut self);
    fn draw_char(&mut self, location: Point2<i64>, character: char);
    fn set_viewport_center(&mut self, viewport_center: Point2<i64>);
}

pub struct RendererImpl {
    root: Rc<RefCell<Root>>,
    viewport_center: Point2<i64>,
}

impl RendererImpl {
    pub fn new(root: Rc<RefCell<Root>>) -> RendererImpl {
        RendererImpl {
            root: root,
            viewport_center: Point2::new(0, 0),
        }
    }

    fn point_to_console(&self, point: Point2<i64>) -> Option<(i32, i32)> {
        let root = self.root.borrow();
        let relative = Point2::new(point.x - self.viewport_center.x,
                                 point.y - self.viewport_center.y);
        let result = Point2::new(relative.x + root.deref().width() as i64 / 2,
                                 relative.y + root.deref().height() as i64 / 2);
        println!("{:?}", result);

        match self.point_is_inside(result) {
            true => Some((result.x as i32, result.y as i32)),
            false => None,
        }
    }

    fn point_is_inside(&self, point: Point2<i64>) -> bool {
        point.x >= 0 && point.x < self.root.borrow().deref().width() as i64
            && point.y >= 0 && point.y < self.root.borrow().deref().height() as i64
    }
}

impl Renderer for RendererImpl {
    fn clear(&mut self) {
        self.root.borrow_mut().deref_mut().clear();
    }

    fn flush(&mut self) {
        self.root.borrow_mut().deref_mut().flush();
    }

    fn draw_char(&mut self, location: Point2<i64>, character: char) {
        match self.point_to_console(location) {
            Some((x, y)) => self.root.borrow_mut().deref_mut().put_char(x, y, character, BackgroundFlag::Default),
            None => (),
        }
    }

    fn set_viewport_center(&mut self, viewport_center: Point2<i64>) {
        self.viewport_center = viewport_center;
    }
}
