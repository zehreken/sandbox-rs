use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
mod particle;
use particle::ParticleModel;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let start = Instant::now();
    let particles= ParticleModel::new(WIDTH, HEIGHT);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("sandbox-rs", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let diff: Duration = Instant::now() - start;
        for (index, i) in buffer.iter_mut().enumerate() {
            *i = (index as u32) << 16 | (index as u32) << 8 | diff.as_millis() as u32;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
