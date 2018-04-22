use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::borrow::Borrow;

use nphysics2d::object::RigidBody;
use nalgebra::{Point2, Isometry2};
use graphics::context::Context;
use graphics::{rectangle, Transformed};
use opengl_graphics::GlGraphics;

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

    pub fn pos(&self) -> Point2<f64> {
        let refcell : &RefCell<RigidBody<f64>> = self.body.borrow();
        let reference : Ref<RigidBody<f64>> = refcell.borrow();
        let matrix : Isometry2<f64> = *reference.position();
        matrix.translation * Point2::new(1.0, 1.0)
    }

    pub fn draw(&self, context : &Context, gl : &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, 50.0);
        let pos = self.pos();
        let transform = context.transform.trans(pos.x, pos.y);
        rectangle(WHITE, square, transform, gl);
    }
}
