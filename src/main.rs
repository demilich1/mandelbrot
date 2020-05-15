use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
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
    min_x_coord: f32,
    max_x_coord: f32,
    min_y_coord: f32,
    max_y_coord: f32,
    offset_x: f32,
    offset_y: f32,
    zoom: f32,
    max_iter: u16,
    width: u16,
    height: u16,
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
            min_x_coord: -2.0,
            max_x_coord: 1.0,
            min_y_coord: -1.0,
            max_y_coord: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            zoom: 1.0,
            max_iter: 100,
            width: width as u16,
            height: height as u16,
            buffer: buf,
        };
        Ok(state)
    }
}

impl EventHandler for MandelbrotSim {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for idx in (0..self.buffer.len()).step_by(4) {
            let idx_n = (idx / 4) as u32;
            let x = idx_n % self.width as u32;
            let y = idx_n / self.width as u32;
            let cx = self.min_x_coord
                + (x as f32 / self.width as f32) * (self.max_x_coord - self.min_x_coord);
            let cy = self.min_y_coord
                + (y as f32 / self.height as f32) * (self.max_y_coord - self.min_y_coord);
            let m = self.mandelbrot(self.offset_x + cx / self.zoom, self.offset_y + cy / self.zoom);
            let color = get_color(m, self.max_iter);

            self.buffer[idx] = color.0;
            self.buffer[idx+1] = color.1;
            self.buffer[idx+2] = color.2;
            self.buffer[idx+3] = 255;
        }

        let image = graphics::Image::from_rgba8(ctx, self.width, self.height, &self.buffer)?;
        //image.set_filter(graphics::FilterMode::Nearest);

        // Draw an image.
        let dst = Point2::new(0.0, 0.0);
        graphics::draw(ctx, &image, (dst,))?;

        self.frames += 1;
        if (self.frames % 10) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        //self.zoom += 0.01;

        // Draw code here...
        graphics::present(ctx)
    }
}

impl MandelbrotSim {
    fn mandelbrot(&self, cx: f32, cy: f32) -> u16 {
        let mut zx = 0.0;
        let mut zy = 0.0;
        let mut n = 0;
        let mut abs = 0.0;
        while abs <= 4.0 && n < self.max_iter {
            let tmp = zx * zx - zy * zy + cx;
            zy = 2.0 * zx * zy + cy;
            zx = tmp;
            n += 1;
            abs = zx * zx + zy * zy;
        }
        n
    }
}

fn get_color(iter:u16, max_iter:u16) -> (u8, u8, u8) {
    let t = iter as f64 / max_iter as f64;
    let rt = 9.0 * (1.0 - t) * t.powi(3);
    let gt = 15.0 * (1.0 - t).powi(2) * t.powi(2);
    let bt = 8.5 * (1.0 - t).powi(3) * t;
    let r = (rt * 255.0) as u8;
    let g = (gt * 255.0) as u8;
    let b = (bt * 255.0) as u8;
    (r, g, b)
}