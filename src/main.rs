use image::{ImageBuffer, Luma, Rgb};
use rand::Rng;

#[derive(PartialEq, Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn calculate_distance(&self, p: &Point) -> u32 {
        let px = (self.x as i32 - p.x as i32).pow(2);
        let py = (self.y as i32 - p.y as i32).pow(2);

        (px as f64 + py as f64).sqrt() as u32
    }
}

pub const WIDTH: u32 = 5000;
pub const HEIGHT: u32 =  4330; // Maintains the proportions of an equilateral triangle (5000 / 4330)

const ITERATIONS: u32 = 5_000_000;

pub const ANGLES: [Point; 3] = [
    Point {x: 0, y: HEIGHT - 1},
    Point {x: WIDTH - 1, y: HEIGHT - 1},
    Point {x: WIDTH / 2, y: 0},
];

fn main() {

    // Setup the image
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if ANGLES.contains(&Point { x, y }) {
            *pixel = Rgb([255_u8, 0, 0]);
        } else {
            *pixel = Rgb([255_u8, 255, 255]);
        }
    }

    // Drawing the fractal
    let mut p = Point {
        x: 0,
        y: HEIGHT - 1,
    }; // The current pixel

    for _ in 0..ITERATIONS {
        let d = rand::thread_rng().gen_range(0..3); // Chooses index of angle
        p.x = (p.x + &ANGLES[d].x) / 2;
        p.y = (p.y + &ANGLES[d].y) / 2;

        img.put_pixel(p.x, p.y, get_color(&p));
    }

    // Saving the image
    img.save("img/triangle.png")
        .expect("Failed to save picture");
}

fn get_color(p: &Point) -> Rgb<u8> {
    let red = 255. - p.calculate_distance(&ANGLES[1]) as f64 * (255. / WIDTH as f64);
    let green = 255. - p.calculate_distance(&ANGLES[2]) as f64 * (255. / WIDTH as f64);
    let blue = 255. - p.calculate_distance(&ANGLES[0]) as f64 * (255. / WIDTH as f64);

    Rgb([red as u8, green as u8, blue as u8])
}