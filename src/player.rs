use std::rc::Rc;
use std::cell::RefCell;

use nphysics2d::object::{RigidBody, Sensor};
use nalgebra::{Isometry2, Point2, Vector2};
use graphics::context::Context;
use graphics::{rectangle, Transformed};
use graphics::color::WHITE;
use opengl_graphics::GlGraphics;

use physics::Physics;

pub const SIZE: f64 = 10.0;

pub struct Player {
    pub body: Rc<RefCell<RigidBody<f64>>>,
    pub sensor: Rc<RefCell<Sensor<f64>>>,
}

impl Player {
    pub fn position(&self) -> Point2<f64> {
        let body = &*Physics::borrow_handle(&self.body);
        let matrix: &Isometry2<f64> = body.position();
        let result = matrix.translation * Point2::new(1.0, 1.0);
        result
    }

    pub fn draw(&self, context: &Context, gl: &mut GlGraphics) {
        let pos = self.position();
        let transform = context.transform.trans(pos.x - SIZE / 2.0, pos.y - SIZE / 2.0);
        let square = rectangle::square(0.0, 0.0, SIZE);
        rectangle(WHITE, square, transform, gl);
    }

    fn can_jump(&self) -> bool {
        let sensor = Physics::borrow_handle(&self.sensor);
        let pos = sensor.position();
        println!("sensor position {:?}", pos);
        let bodies = sensor.interfering_bodies();
        !bodies.is_empty()
    }

    pub fn mov(&mut self, dx: f64, jump: bool) {
        let body = &mut *self.body.borrow_mut();
        if self.can_jump() && jump {
            body.apply_central_impulse(Vector2::new(0.0, -700.0));
        } 
        let dy = body.lin_vel()[1];
        body.set_lin_vel(Vector2::new(100.0 * dx, dy));
    }
}
