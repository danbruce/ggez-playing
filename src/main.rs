extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use rand::random;
use std::time::Duration;

// First we make a structure to contain the game's state
struct MainState {
    ball_x: f32,
    ball_dx: f32,
    ball_y: f32,
    ball_dy: f32,
    fps: graphics::Text,
    fps_font: graphics::Font,
    frames: u8
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let fps_font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 16)?;
        let fps = graphics::Text::new(ctx, "0 FPS", &fps_font)?;
        let s = MainState {
            ball_x: 10.0,
            ball_dx: random::<f32>() * 20.0 - 10.0,
            ball_y: 10.0,
            ball_dy: random::<f32>() * 20.0 - 10.0,
            fps: fps,
            fps_font: fps_font,
            frames: 0
        };
        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;
        if self.ball_x <= 0.0 || self.ball_x >= 800.0 {
            self.ball_dx *= -1.0;            
        }
        if self.ball_y <= 0.0 || self.ball_y >= 600.0 {
            self.ball_dy *= -1.0;
        }
        self.frames += 1;
        if self.frames >= 60 {
            self.frames = 0;
            self.fps = {
                let fps_string = format!("{:.*} FPS", 1, ggez::timer::get_fps(ctx));
                graphics::Text::new(ctx, &fps_string, &self.fps_font)?
            };
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::BLACK)?;
        let rect = graphics::Rect::new(0.0, 0.0, 1600.0, 1200.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
        graphics::set_color(ctx, graphics::WHITE)?;
        let ball_loc = graphics::Point::new(self.ball_x, self.ball_y);
        graphics::ellipse(ctx, graphics::DrawMode::Fill, ball_loc, 15.0, 15.0, 32)?;
        let fps_point = graphics::Point::new(
            800.0 - self.fps.width() as f32 / 2.0 - 5.0,
            600.0 - self.fps.height() as f32 / 2.0 - 5.0,
        );
        graphics::set_color(ctx, graphics::Color::new(1.0, 0.0, 0.0, 0.65))?;
        graphics::draw(ctx, &self.fps, fps_point, 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title,
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game,
// * then just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    let mut c = conf::Conf::new();
    c.resizable = true;
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}