extern crate nalgebra;
extern crate ncollide;
extern crate nphysics2d;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::HashSet;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::color::BLACK;

mod player;
mod physics;
use physics::Physics;
mod draw;

pub struct App {
    gl: GlGraphics,
    down_keys: HashSet<Key>,
    cursor_x: f64,
    cursor_y: f64,
    physics: Physics,
}

impl App {
    fn new(gl: GlGraphics) -> App {
        App {
            gl,
            down_keys: HashSet::new(),
            physics: Physics::new(),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // clear the screen
        self.gl.draw(args.viewport(), |_context, gl| {
            clear(BLACK, gl);
        });
        self.physics.draw(&args.viewport(), &mut self.gl, self.cursor_x, self.cursor_y);
    }

    fn update(&mut self, args: &UpdateArgs) {
        let dt = args.dt;
        let mut jump = false;
        let mut dx: f64 = 0.0;
        for &key in &self.down_keys {
            match key {
                Key::A => dx -= 1.0,
                Key::D => dx += 1.0,
                Key::Space => jump = true,
                _ => (),
            }
        }
        self.physics.update(dt, dx, jump);
    }

    fn handle_button(&mut self, args: &ButtonArgs) {
        if let Button::Keyboard(key) = args.button {
            if args.state == ButtonState::Press {
                self.down_keys.insert(key);
            } else {
                self.down_keys.remove(&key);
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "A Light in the Dark",
        [physics::WIDTH as u32, physics::HEIGHT as u32],
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Input(Input::Button(args)) => app.handle_button(&args),
            Event::Loop(Loop::Update(args)) => app.update(&args),
            Event::Loop(Loop::Render(args)) => app.render(&args),
            _ => (),
        }
        e.mouse_cursor(|x, y| {
            cursor_x = x;
            cursor_y = y;
        });
    }
}
