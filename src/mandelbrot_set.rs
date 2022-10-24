// Mandelbrot set is used to refer to both a general class of fractal sets and to a particular instance
// of such a set. In general, a Mandelbrot set marks the set of points in the complex plane such that the
// corresponding Julia set is connected and not computable. The set was first defined and drawn by Robert
// W. Brooks and Peter Matelski in 1978, however in 1980 Benoit Mandelbrot obtained high quality visualizations 
// of the set while working at IBM's Thomas J. Watson Research Center. 

// The Mandelbrot set has become popular outside of mathematics both for its aesthetic appeal and as an 
// example of a complex structure arising from the application of simple rules.

use num::complex::Complex;

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);

    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);

        for img_x in 0..width {
            let x_percent = (img_x as f64 / width as f64);
            let y_percent = (img_y as f64 / height as f64);
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }

    rows
}

fn mandelbrot_at_point(
    cx: f64,
    cy: f64,
    max_iters: usize,
) -> usize {
    // Initializing a complex number at the origin with real and imaginary parts at 0.0
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }

        z = z * z + c;
    }

    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => " ",
                2..=5 => ".",
                5..=10 => "•",
                11..=30 => "*",
                30..=100 => "+",
                100..=200 => "x",
                200..=400 => "$",
                400..=700 => "#",
                _ => "%",
            };

            line.push(val);
        }

        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, 2.0, 1.0, -1.0, 1.0, 100, 24);
    render_mandelbrot(mandelbrot);
}