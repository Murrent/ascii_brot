use num_complex::Complex;
use std::time::Instant;

const MAX_ITER: u32 = 1000;
const X_OFFSET: i32 = -500;
const Y_OFFSET: i32 = 0;

fn mandelbrot(c: Complex<f64>) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..MAX_ITER {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn main() {
    let start_time = Instant::now();

    let (width, height) = (2200, 420);
    let width_f64 = width as f64;
    let height_f64 = height as f64;
    let scale_x = width_f64 / 3.0;
    let scale_y = height_f64 / 2.3;

    for y in 0..height {
        let y_f64 = (y as i32 + Y_OFFSET) as f64;
        let im = (y_f64 - height_f64 / 2.0) / scale_y;
        for x in 0..width {
            let x_f64 = (x as i32 + X_OFFSET) as f64;
            let re = (x_f64 - width_f64 / 2.0) / scale_x;
            let c = Complex { re, im };

            match mandelbrot(c) {
                Some(i) => {
                    let i_u8 = (33 + i % 61) as u8;
                    print!("{}", i_u8 as char);
                }
                None => print!(" "),
            }
        }
        println!("");
    }

    let duration = start_time.elapsed().as_secs_f64();
    println!("Time elapsed is: {:.4} seconds", duration);
}
