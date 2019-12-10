#![feature(drain_filter)]

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::conf::WindowMode;

mod cars;
mod borders;
mod game_maths;
mod interval;

use crate::cars::Car;
use crate::borders::Border;
use crate::game_maths::Vec2;
use std::sync::{RwLock, Arc};
use ggez::input::mouse::MouseButton;
use crate::interval::Timer;
use ggez::input::keyboard::KeyCode;
use std::borrow::Borrow;

const SECOND_UPDATE: usize = 123;

fn main() -> GameResult<()> {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_AUTHORS"))
        .window_mode(WindowMode {
            width: 1920.0,
            height: 1080.0,
            .. WindowMode::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = Sim::new(&mut ctx)?;

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    };

    Ok(())
}

struct Sim {
    // Your state here...
    cars: Vec<Box<Car>>,
    boundary: Arc<RwLock<Border>>,
    view_offset: Vec2,
    view_scale: f32,
    last_mouse_pos: Vec2,
    timer: Timer
}

impl Sim {
    pub fn new(ctx: &mut Context) -> GameResult<Sim> {
        // Load/create resources such as images here.

        // testing cars
        let mut cars = vec![
            Car::new((0.0, 0.0).into(), 0.0, ctx),
        ];

        let mut timer = Timer::new();

        timer.add(SECOND_UPDATE, 1.0, true);

        Ok(Sim {
            cars: cars.drain(..).map(|e| Box::new(e)).collect(),
            boundary: Arc::new(RwLock::new(Border::new(ctx, 15, 40.0, 15.0, 10.0)?)),
            view_offset: Vec2::new(0.0, 0.0),
            view_scale: 10.0,
            last_mouse_pos: Vec2::new(0.0, 0.0),
            timer
        })
    }
}

impl EventHandler for Sim {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...

        for timer_id in self.timer.update(ggez::timer::delta(ctx).as_secs_f32()) {
            match timer_id {
                SECOND_UPDATE => {
                    println!("FPS: {}", ggez::timer::fps(ctx));
                },
                _ => {}
            }
        }

        let mut car = &mut self.cars[0];

        let speed = if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::W) {1f32}
        else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::S) {-1f32} else {0.0};

        let steering = if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::A) {-0.1}
        else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::D) {0.1} else {0.0};
        car.update(speed, steering, &(*self.boundary).read().unwrap());


        std::thread::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let mut center: Vec2 = graphics::size(ctx).into();
        center *= 0.5;
        let view_offset = center + self.view_offset * self.view_scale;

        (*self.boundary.read().unwrap()).draw(ctx, view_offset, self.view_scale)?;

        //println!("{:?}", center);

        for car in &mut self.cars {
            car.draw(ctx, view_offset, self.view_scale)?;
        }
        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let m_pos = Vec2::new(x, y);
        if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
            let move_delta = m_pos - self.last_mouse_pos;
            self.view_offset += move_delta * (1.0 / self.view_scale);
        }
        self.last_mouse_pos = m_pos;
    }


    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        if self.view_scale >= 1.0 {
            self.view_scale += y * (self.view_scale / 2.0).sqrt();
        }
        else {
            self.view_scale += y * (self.view_scale * 0.5f32.sqrt());
        }
        println!("scale: {}  | y: {}", self.view_scale, y);
    }
}