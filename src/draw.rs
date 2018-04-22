use nphysics2d::object::RigidBody;

use graphics::Context;
use opengl_graphics::GlGraphics;

pub trait Draw {
    fn draw(&self, &RigidBody<f64>, &Context, &mut GlGraphics);
}
