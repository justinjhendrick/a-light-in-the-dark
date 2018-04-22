use nalgebra::{Vector2, Translation2};
use ncollide::shape::{Ball, Plane, Cuboid};
use nphysics2d::world::World;
use nphysics2d::object::RigidBody;

use opengl_graphics::GlGraphics;
use graphics::Viewport;

use player::{Player, SIZE};

pub const HEIGHT : f64 = 600.0;
pub const WIDTH : f64 = 600.0;

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
            Plane::new(Vector2::new(0.0, 1.0)),
            restitution,
            friction
        );
        ceiling.append_translation(&Translation2::new(0.0, 0.0));
        world.add_rigid_body(ceiling);

        let mut floor = RigidBody::new_static(
            Plane::new(Vector2::new(0.0, -1.0)),
            restitution,
            friction
        );
        floor.append_translation(&Translation2::new(0.0, HEIGHT));
        world.add_rigid_body(floor);
    }

    fn add_player(world : &mut World<f64>) -> Player {
        let init_x = 0.0;
        let init_y = 0.0;
        let shape = Cuboid::new(Vector2::new(SIZE, SIZE));
        let density = 0.1;
        let restitution = 0.3;
        let friction = 0.6;

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

    pub fn draw(&self, viewport : &Viewport, gl : &mut GlGraphics) {
        gl.draw(*viewport, |context, ref mut gl| {
            self.player.draw(&context, gl);
        });
    }
}
