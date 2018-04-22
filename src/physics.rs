use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::borrow::Borrow;

use nalgebra::{Vector2, Translation2, Point2};
use ncollide::shape::{Shape2, Ball, Cuboid};
use ncollide::query::{Ray, RayCast};
use nphysics2d::world::World;
use nphysics2d::object::{RigidBody};

use opengl_graphics::GlGraphics;
use graphics::{Viewport, Context, rectangle, Transformed};
use graphics::color::WHITE;

use player::{Player, SIZE};

pub const HEIGHT : f64 = 600.0;
pub const WIDTH : f64 = 600.0;

pub const RED : [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct Physics {
    world : World<f64>,
    player : Player,
}

impl Physics {
    pub fn new() -> Physics {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 9.81));
        Physics::add_static_bodies(&mut world);
        let player = Physics::add_player(&mut world);
        Physics {
            world,
            player,
        }
    }

    // floor, ceiling, map, etc.
    fn add_static_bodies(world : &mut World<f64>) {
        let restitution = 0.3;
        let friction = 0.6;

        let mut ceiling = RigidBody::new_static(
            Cuboid::new(Vector2::new(WIDTH / 2.0, 1.0)),
            restitution,
            friction
        );
        ceiling.append_translation(&Translation2::new(WIDTH / 2.0, 3.0));
        world.add_rigid_body(ceiling);

        let mut right_wall = RigidBody::new_static(
            Cuboid::new(Vector2::new(1.0, HEIGHT / 2.0)),
            restitution,
            friction
        );
        right_wall.append_translation(&Translation2::new(WIDTH - 3.0, HEIGHT / 2.0));
        world.add_rigid_body(right_wall);

        let mut floor = RigidBody::new_static(
            Cuboid::new(Vector2::new(WIDTH / 2.0, 1.0)),
            restitution,
            friction
        );
        floor.append_translation(&Translation2::new(WIDTH / 2.0, HEIGHT - 3.0));
        world.add_rigid_body(floor);

        let mut left_wall = RigidBody::new_static(
            Cuboid::new(Vector2::new(1.0, HEIGHT / 2.0)),
            restitution,
            friction
        );
        left_wall.append_translation(&Translation2::new(3.0, HEIGHT / 2.0));
        world.add_rigid_body(left_wall);
    }

    fn add_player(world : &mut World<f64>) -> Player {
        let init_x = WIDTH / 2.0;
        let init_y = HEIGHT / 2.0;
        let shape = Ball::new(SIZE);
        let density = 0.1;
        let restitution = 0.3;
        let friction = 1.2;

        let mut rb =
          RigidBody::new_dynamic(shape, density, restitution, friction);
        rb.append_translation(&Translation2::new(init_x, init_y));
        let player_body = world.add_rigid_body(rb);
        Player::new(player_body)
    }

    pub fn update(&mut self, dt : f64, dx : f64, jump : bool) {
        self.player.mov(dx, jump);
        self.world.step(dt);
    }

    pub fn draw(&mut self, viewport : &Viewport, gl : &mut GlGraphics) {
        gl.draw(*viewport, |context, ref mut gl| {
            self.player.draw(&context, gl);

            let mouse_position = Vector2::new(WIDTH / 2.0, HEIGHT / 2.0);
            self.draw_rays(&mouse_position, &context, gl);
        });
    }

    fn draw_rays(&mut self, mouse_position : &Vector2<f64>, context : &Context, gl : &mut GlGraphics) {
        let ray = Ray::new(self.player.position(), *mouse_position);
        for body in self.world.rigid_bodies() {
            let body = &*Physics::borrow_handle(body);
            let shape = body.shape().as_ref();
            let transform = body.position();
            //if shape.intersects_ray(transform, &ray) {
            // physics points are center
            let physics_pos = transform * Point2::new(1.0, 1.0);

            // convert the physics rectangle into a graphics rectangle
            let rect = match Shape2::as_shape::<Cuboid<Vector2<f64>>>(shape) {
                Some(v) => v,
                None => continue,
            };
            let rect_size : Vector2<f64> = *rect.half_extents();
            let half_width = rect_size[0];
            let half_height = rect_size[1];
            let graphics_shape = [0.0, 0.0, 2.0 * half_width, 2.0 * half_height];
            // graphics points are top left
            let graphics_transform = context.transform.trans(physics_pos.x - half_width, physics_pos.y - half_height);
            rectangle(RED, graphics_shape, graphics_transform, gl);
        }
    }

    pub fn borrow_handle<'a, T>(h : &'a Rc<RefCell<T>>) -> Ref<'a, T> {
        let refcell : &RefCell<T> = h.borrow();
        let reference : Ref<T> = refcell.borrow();
        reference
    }
}
