mod mandelbrot_renderer;

use mandelbrot_renderer::MandelbrotRenderParams;

use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::input;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, ContextBuilder, GameResult};

fn main() -> GameResult {
    let window_setup = WindowSetup::default().title("Mandelbrot");
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("mandelbrot", "demilich")
        .window_setup(window_setup)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut mandelbrot_sim = MandelbrotSim::new(&mut ctx)?;

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut mandelbrot_sim) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
    Ok(())
}

struct MandelbrotSim {
    frames: i32,
    params: MandelbrotRenderParams,
    buffer: Vec<u8>,
}

impl MandelbrotSim {
    pub fn new(_ctx: &mut Context) -> GameResult<MandelbrotSim> {
        // Load/create resources such as images here.
        let width = 800;
        let height = 600;
        let buf = vec![0; width * height * 4];

        let state = MandelbrotSim {
            frames: 0,
            params: MandelbrotRenderParams::new(width, height),
            buffer: buf,
        };
        Ok(state)
    }
}

impl EventHandler for MandelbrotSim {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.params.offset_x -= ggez::timer::delta(ctx).as_secs_f64() * 0.2 / self.params.zoom;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.params.offset_x += ggez::timer::delta(ctx).as_secs_f64() * 0.2 / self.params.zoom;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.params.offset_y -= ggez::timer::delta(ctx).as_secs_f64() * 0.2 / self.params.zoom;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.params.offset_y += ggez::timer::delta(ctx).as_secs_f64() * 0.2 / self.params.zoom;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.params.zoom += ggez::timer::delta(ctx).as_secs_f64() * 0.1 * self.params.zoom;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.params.zoom -= ggez::timer::delta(ctx).as_secs_f64() * 0.1 * self.params.zoom;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.params.max_iter -= 10;
            if self.params.max_iter < 10 {
                self.params.max_iter = 10;
            }
            println!("Max iterations set to {}", self.params.max_iter);
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.params.max_iter += 10;
            println!("Max iterations set to {}", self.params.max_iter);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        mandelbrot_renderer::render_to_buffer(&self.params, &mut self.buffer);

        let image =
            graphics::Image::from_rgba8(ctx, self.params.width, self.params.height, &self.buffer)?;
        //image.set_filter(graphics::FilterMode::Nearest);

        let dst = Point2::new(0.0, 0.0);
        graphics::draw(ctx, &image, (dst,))?;

        self.frames += 1;
        if (self.frames % 10) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        graphics::present(ctx)
    }
}
