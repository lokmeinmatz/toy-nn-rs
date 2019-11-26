use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::conf::WindowMode;

mod cars;
mod borders;
mod game_maths;

use crate::cars::Car;
use crate::borders::Border;
use crate::game_maths::Vec2;
use std::sync::{RwLock, Arc};

fn main() {
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
    let mut my_game = Sim::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Sim {
    // Your state here...
    cars: Vec<Box<Car>>,
    boundary: Arc<RwLock<Border>>
}

impl Sim {
    pub fn new(_ctx: &mut Context) -> Sim {
        // Load/create resources such as images here.
        Sim {
            cars: Vec::new(),
            boundary: Arc::new(RwLock::new(Border{}))
        }
    }
}

impl EventHandler for Sim {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // Draw code here...
        graphics::present(ctx)
    }
}