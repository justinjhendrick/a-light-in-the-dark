use std::rc::Rc;
use std::any::Any;
use std::borrow::Borrow;

use nalgebra::{Vector2, Translation2, Point2};
use ncollide::shape::{Ball, Plane};
use nphysics2d::world::World;
use nphysics2d::object::RigidBody;

use opengl_graphics::GlGraphics;
use graphics::Viewport;

use player::Player;
use draw::Draw;

pub struct Physics {
    world : World<f64>,
}

impl Physics {
    pub fn new() -> Physics {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 9.81));
        Physics::add_static_bodies(&mut world);
        Physics::add_player(&mut world);
        Physics {
            world,
        }
    }

    // floor, ceiling, map, etc.
    fn add_static_bodies(world : &mut World<f64>) {
        let restitution = 0.3;
        let friction = 0.6;

        let mut ceiling = RigidBody::new_static(
            Plane::new(Vector2::new(0.0, 1.0)),
            restitution,
            friction
        );
        ceiling.append_translation(&Translation2::new(0.0, -10.0));
        world.add_rigid_body(ceiling);

        let mut floor = RigidBody::new_static(
            Plane::new(Vector2::new(0.0, -1.0)),
            restitution,
            friction
        );
        floor.append_translation(&Translation2::new(0.0, 10.0));
        world.add_rigid_body(floor);
    }

    fn add_player(world : &mut World<f64>) {
        let init_x = 0.0;
        let init_y = 0.0;
        let shape = Ball::new(2.5);
        let density = 1.0;
        let restitution = 0.3;
        let friction = 0.6;

        let mut rb =
          RigidBody::new_dynamic(shape, density, restitution, friction);
        rb.append_translation(&Translation2::new(init_x, init_y));
        rb.set_user_data(Some(Box::new(Player::new())));
        world.add_rigid_body(rb);
    }

    pub fn update(&self, dt : f64, dx : f64, jump : bool) {
        // TODO
    }

    pub fn draw(&self, viewport : &Viewport, gl : &mut GlGraphics) {
        for body in self.world.rigid_bodies() {
            let body : &RigidBody<f64> = &match Rc::get_mut(body) {
                Ok(b) => b,
                Err(_) => continue,
            }.into_inner();
            gl.draw(*viewport, |context, ref mut gl| {
                match body.user_data() {
                    Some(drawable) => {
                        let drawable : &Any = drawable.borrow();
                        if ((*drawable).is::<Player>()) {
                            let player : &Player = (*drawable).downcast_ref().unwrap();
                            player.draw(&body, &context, &mut gl);
                        }
                    }
                    None => (),
                }
            });
        }
    }
}
