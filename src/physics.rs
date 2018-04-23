use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::borrow::Borrow;

use nalgebra::{Point2, Translation2, Vector2, Isometry2};
use ncollide::shape::{Ball, Cuboid, Shape2};
use ncollide::query::{Ray, RayCast};
use nphysics2d::world::World;
use nphysics2d::object::{RigidBody, Sensor};

use opengl_graphics::GlGraphics;
use graphics::{rectangle, Context, Transformed, Viewport};
use graphics::color::WHITE;

use player::{Player, SIZE};

pub const HEIGHT: f64 = 600.0;
pub const WIDTH: f64 = 600.0;

pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct Physics {
    world: World<f64>,
    player: Player,
}

impl Physics {
    pub fn new() -> Physics {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 500.0));
        Physics::add_static_bodies(&mut world);
        let player = Physics::add_player(&mut world);
        Physics { world, player }
    }

    // floor, ceiling, map, etc.
    fn add_static_bodies(world: &mut World<f64>) {
        let restitution = 0.3;
        let friction = 0.6;

        let mut ceiling = RigidBody::new_static(
            Cuboid::new(Vector2::new(WIDTH / 2.0, 1.0)),
            restitution,
            friction,
        );
        ceiling.append_translation(&Translation2::new(WIDTH / 2.0, 3.0));
        world.add_rigid_body(ceiling);

        let mut right_wall = RigidBody::new_static(
            Cuboid::new(Vector2::new(1.0, HEIGHT / 2.0)),
            restitution,
            friction,
        );
        right_wall.append_translation(&Translation2::new(WIDTH - 3.0, HEIGHT / 2.0));
        world.add_rigid_body(right_wall);

        let mut floor = RigidBody::new_static(
            Cuboid::new(Vector2::new(WIDTH / 2.0, 1.0)),
            restitution,
            friction,
        );
        floor.append_translation(&Translation2::new(WIDTH / 2.0, HEIGHT - 3.0));
        world.add_rigid_body(floor);

        let mut left_wall = RigidBody::new_static(
            Cuboid::new(Vector2::new(2.0, HEIGHT / 2.0)),
            restitution,
            friction,
        );
        left_wall.append_translation(&Translation2::new(5.0, HEIGHT / 2.0));
        world.add_rigid_body(left_wall);

        let mut platform = RigidBody::new_static(
            Cuboid::new(Vector2::new(WIDTH / 8.0, 10.0)),
            restitution,
            friction,
        );
        platform.append_translation(&Translation2::new(WIDTH / 2.0, HEIGHT * 4.0 / 5.0));
        world.add_rigid_body(platform);
    }

    fn add_player(world: &mut World<f64>) -> Player {
        let init_x = WIDTH / 2.0;
        let init_y = HEIGHT / 2.0;
        let density = 0.1;
        let restitution = 0.3;
        let friction = 1.2;

        let mut rb = RigidBody::new_dynamic(Ball::new(SIZE), density, restitution, friction);
        rb.append_translation(&Translation2::new(init_x, init_y));
        let player_body_handle = world.add_rigid_body(rb);
        let mut sensor = Sensor::new(Ball::new(10.0 * SIZE), Some(player_body_handle.clone()));
        sensor.set_relative_position(Isometry2::new(Vector2::new(0.0, 0.0), 0.0));
        let sensor_handle = world.add_sensor(sensor);
        Player {
            body: player_body_handle,
            sensor: sensor_handle,
        }
    }

    pub fn update(&mut self, dt: f64, dx: f64, jump: bool) {
        self.player.mov(dx, jump);
        self.world.step(dt);
    }

    pub fn draw(&mut self, viewport: &Viewport, gl: &mut GlGraphics, cursor_x: f64, cursor_y: f64) {
        gl.draw(*viewport, |context, ref mut gl| {
            self.player.draw(&context, gl);

            self.draw_rays(&context, gl, cursor_x, cursor_y);
        });
    }

    // returns a vector pointing from a towards b (of unit length)
    fn normal_difference(a_x: f64, a_y: f64, b_x: f64, b_y: f64) -> Vector2<f64> {
        let x = b_x - a_x;
        let y = b_y - a_y;
        let norm = (x * x + y * y).sqrt();
        Vector2::new(x / norm, y / norm)
    }

    fn draw_rays(&mut self, context: &Context, gl: &mut GlGraphics, cursor_x: f64, cursor_y: f64) {
        let player_pos = self.player.position();
        let ray_dir = Physics::normal_difference(player_pos[0], player_pos[1], cursor_x, cursor_y);
        let ray = Ray::new(player_pos, ray_dir);
        for body in self.world.rigid_bodies() {
            let body = &*Physics::borrow_handle(body);
            let shape = body.shape().as_ref();
            let transform = body.position();
            if shape.intersects_ray(transform, &ray) {
                // physics points are center
                let physics_pos = transform * Point2::new(1.0, 1.0);

                // convert the physics rectangle into a graphics rectangle
                let rect = match Shape2::as_shape::<Cuboid<Vector2<f64>>>(shape) {
                    Some(v) => v,
                    None => continue,
                };
                let rect_size: Vector2<f64> = *rect.half_extents();
                let half_width = rect_size[0];
                let half_height = rect_size[1];
                let graphics_shape = [0.0, 0.0, 2.0 * half_width, 2.0 * half_height];
                // graphics points are top left
                let graphics_transform = context
                    .transform
                    .trans(physics_pos.x - half_width, physics_pos.y - half_height);
                rectangle(RED, graphics_shape, graphics_transform, gl);
            }
        }
    }

    pub fn borrow_handle<'a, T>(h: &'a Rc<RefCell<T>>) -> Ref<'a, T> {
        let refcell: &RefCell<T> = h.borrow();
        let reference: Ref<T> = refcell.borrow();
        reference
    }
}
