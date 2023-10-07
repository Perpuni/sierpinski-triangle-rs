use image::{ImageBuffer, Luma};
use rand::Rng;

#[derive(PartialEq, Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

pub const WIDTH: u32 = 5000;
pub const HEIGHT: u32 =  4330; // Maintains the proportions of an equilateral triangle

const ITERATIONS: u32 = 5_000_000;

fn main() {
    let angles = [
        Point {x: 0, y: HEIGHT - 1},
        Point {x: WIDTH - 1, y: HEIGHT - 1},
        Point {x: WIDTH / 2, y: 0},
    ];

    let black_pixel = Luma([0_u8]);

    // Setup the image
    let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if angles.contains(&Point { x, y }) {
            *pixel = black_pixel;
        } else {
            *pixel = Luma([255_u8]);
        }
    }

    // Drawing the fractal
    let mut p = Point {
        x: 0,
        y: HEIGHT - 1,
    }; // The current pixel

    for _ in 0..ITERATIONS {
        let d = rand::thread_rng().gen_range(0..3); // Chooses index of angle
        p.x = (p.x + &angles[d].x) / 2;
        p.y = (p.y + &angles[d].y) / 2;

        img.put_pixel(p.x, p.y, black_pixel);
    }

    // Saving the image
    img.save("img/triangle.png")
        .expect("Failed to save picture");
}
