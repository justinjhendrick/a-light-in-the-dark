use std::rc::Rc;
use std::cell::RefCell;

use nphysics2d::object::RigidBody;
use nalgebra::{Isometry2, Point2, Vector2};
use graphics::context::Context;
use graphics::{rectangle, Transformed};
use graphics::color::WHITE;
use opengl_graphics::GlGraphics;

use physics::Physics;

pub const SIZE: f64 = 10.0;

pub struct Player {
    body: Rc<RefCell<RigidBody<f64>>>,
}

impl Player {
    pub fn new(body: Rc<RefCell<RigidBody<f64>>>) -> Player {
        Player { body }
    }

    pub fn position(&self) -> Point2<f64> {
        let body = &*Physics::borrow_handle(&self.body);
        let matrix: &Isometry2<f64> = body.position();
        let result = matrix.translation * Point2::new(1.0, 1.0);
        result
    }

    pub fn draw(&self, context: &Context, gl: &mut GlGraphics) {
        let pos = self.position();
        let transform = context.transform.trans(pos.x, pos.y);
        let square = rectangle::square(0.0, 0.0, SIZE);
        rectangle(WHITE, square, transform, gl);
    }

    pub fn mov(&mut self, dx: f64, jump: bool) {
        let body = &mut *self.body.borrow_mut();
        let dy = if jump { -10.0 } else { 0.0 };
        let scale = 10.0;
        body.apply_central_impulse(Vector2::new(scale * dx, scale * dy));
    }
}
