use rayon::prelude::*;

pub struct MandelbrotRenderParams {
    pub min_x_coord: f64,
    pub max_x_coord: f64,
    pub min_y_coord: f64,
    pub max_y_coord: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub zoom: f64,
    pub max_iter: u16,
    pub width: u16,
    pub height: u16,
}

impl MandelbrotRenderParams {
    pub const DEFAULT_ITER: u16 = 256;
    pub const MIN_ITER: u16 = 16;
    pub const MAX_ITER: u16 = 8192;

    pub fn new(width: usize, height: usize) -> Self {
        MandelbrotRenderParams {
            min_x_coord: -2.0,
            max_x_coord: 1.0,
            min_y_coord: -1.0,
            max_y_coord: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            zoom: 1.0,
            max_iter: MandelbrotRenderParams::DEFAULT_ITER,
            width: width as u16,
            height: height as u16,
        }
    }
}

pub fn render_to_buffer(params: &MandelbrotRenderParams, buffer: &mut [u8]) {
    buffer
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(idx, chunk)| {
            let idx_n = idx as u32;
            let x = idx_n % params.width as u32;
            let y = idx_n / params.width as u32;
            let cx = params.min_x_coord
                + (x as f64 / params.width as f64) * (params.max_x_coord - params.min_x_coord);
            let cy = params.min_y_coord
                + (y as f64 / params.height as f64) * (params.max_y_coord - params.min_y_coord);
            let m = mandelbrot(
                params.offset_x + cx / params.zoom,
                params.offset_y + cy / params.zoom,
                params.max_iter,
            );
            let color = get_color(m, params.max_iter);
            chunk[0] = color[0];
            chunk[1] = color[1];
            chunk[2] = color[2];
            chunk[3] = 255;
        });
}

fn mandelbrot(cx: f64, cy: f64, max_iter: u16) -> u16 {
    let mut zx: f64 = 0.0;
    let mut zy: f64 = 0.0;
    let mut n = 0;
    for _i in 0..max_iter {
        let zxn = zx.powi(2) - zy.powi(2) + cx;
        zy = 2.0 * zx * zy + cy;
        zx = zxn;
        n += 1;
        if zx.powi(2) + zy.powi(2) > 4.0 {
            break;
        }
    }
    n
}

fn get_color(iter: u16, max_iter: u16) -> [u8; 3] {
    let t = iter as f64 / max_iter as f64;
    let rt = 9.0 * (1.0 - t) * t.powi(3);
    let gt = 15.0 * (1.0 - t).powi(2) * t.powi(2);
    let bt = 8.5 * (1.0 - t).powi(3) * t;
    let r = (rt * 255.0) as u8;
    let g = (gt * 255.0) as u8;
    let b = (bt * 255.0) as u8;
    [r, g, b]
}
