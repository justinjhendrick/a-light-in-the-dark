use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use std::borrow::Borrow;

use nphysics2d::object::RigidBody;
use nalgebra::{Point2, Isometry2, Vector2};
use graphics::context::Context;
use graphics::{Transformed, rectangle};
use opengl_graphics::GlGraphics;

pub const SIZE : f64 = 10.0;
const WHITE : [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Player {
    body : Rc<RefCell<RigidBody<f64>>>
}

impl Player {
    pub fn new(body : Rc<RefCell<RigidBody<f64>>>) -> Player {
        Player {
            body,
        }
    }

    fn get_body(&self) -> Ref<RigidBody<f64>> {
        let refcell : &RefCell<RigidBody<f64>> = self.body.borrow();
        let reference : Ref<RigidBody<f64>> = refcell.borrow();
        reference
    }

    fn get_body_mut(&self) -> RefMut<RigidBody<f64>> {
        let refmut : RefMut<RigidBody<f64>> = self.body.borrow_mut();
        refmut
    }

    pub fn pos(&self) -> Point2<f64> {
        let body = &*self.get_body();
        let matrix : &Isometry2<f64> = body.position();
        let result = matrix.translation * Point2::new(1.0, 1.0);
        //println!("pos ({}, {})", result.x, result.y);
        result
    }

    pub fn draw(&self, context : &Context, gl : &mut GlGraphics) {
        let pos = self.pos();
        let transform = context.transform.trans(pos.x, pos.y);
        let square = rectangle::square(0.0, 0.0, SIZE);
        rectangle(WHITE, square, transform, gl);
    }

    pub fn mov(&mut self, dx : f64, jump : bool) {
        let body = &mut *self.get_body_mut();
        let dy = if jump { -10.0 } else { 0.0 };
        let scale = 10.0;
        body.apply_central_impulse(Vector2::new(scale * dx, scale * dy));
    }
}
