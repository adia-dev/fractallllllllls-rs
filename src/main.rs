mod complex;
mod fractals;
use fractals::{Fractal, Julia, Mandlebrot, BurningShip};
use image;

fn main() {
    let image_width = 1920;
    let image_height = 1080;

    let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let u = x as f32 / image_height as f32;
        let v = y as f32 / image_height as f32;

        let t = BurningShip::generate(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
        *pixel = image::Rgb(Fractal::color((2.0 * t + 0.5) % 1.0));
    }

    image_buffer.save("burning_ship.png").unwrap();
}
