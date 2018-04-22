use nphysics2d::object::RigidBody;
use nalgebra::{Translation2, Point2};
use graphics::context::Context;
use graphics::{rectangle, Transformed};
use opengl_graphics::GlGraphics;

const WHITE : [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Player {
}

impl Player {
    pub fn new() -> Player {
        Player {}
    }

    pub fn pos(body : &RigidBody<f64>) -> Point2<f64> {
        let matrix = body.position();
        matrix.translation * Point2::new(1.0, 1.0)
    }

    pub fn draw(&self, body : &RigidBody<f64>, context : &Context, gl : &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, 50.0);
        let pos = Player::pos(body);
        let transform = context.transform.trans(pos.x, pos.y);
        rectangle(WHITE, square, transform, gl);
    }
}
