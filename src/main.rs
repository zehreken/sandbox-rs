use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::time::{Duration, Instant};
mod particle;
use particle::{ParticleModel, ParticleProperties};

const WIDTH: usize = 320;
const HEIGHT: usize = 180;

fn main() {
    // let start = Instant::now();
    let mut model = ParticleModel::new(WIDTH, HEIGHT);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "sandbox-rs",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // let diff: Duration = Instant::now() - start;

        // model.particles[160] = Some(Particle::water());

        window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
            let screen_pos = y as usize * WIDTH + x as usize;

            if window.get_mouse_down(MouseButton::Left) {
                model.particles[screen_pos] = Some(ParticleProperties::sand());
            }

            if window.get_mouse_down(MouseButton::Right) {
                model.particles[screen_pos] = Some(ParticleProperties::water());
            }
        });

        for (index, i) in buffer.iter_mut().enumerate() {
            if let Some(p) = &model.particles[index] {
                *i = p.color;
            } else {
                *i = 0;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        model.simulate();
    }
}
