use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::time::Duration;
mod particle;
use particle::{Particle, ParticleModel, Rock, Sand, Water};

const WIDTH: usize = 160;
const HEIGHT: usize = 90;

fn main() {
    // let start = Instant::now();
    let mut model = ParticleModel::new(WIDTH, HEIGHT);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut selected: u8 = 1;

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

        window.get_keys_released().map(|keys| {
            for t in keys {
                match t {
                    Key::Key1 => selected = 1,
                    Key::Key2 => selected = 2,
                    Key::Key3 => selected = 3,
                    Key::C => model.clear(),
                    _ => (),
                }
            }
        });

        window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
            let screen_pos = y as usize * WIDTH + x as usize;

            if window.get_mouse_down(MouseButton::Left) {
                match selected {
                    1 => model._particles[screen_pos] = Some(Box::new(Sand::new())),
                    2 => model._particles[screen_pos] = Some(Box::new(Water::new())),
                    3 => model._particles[screen_pos] = Some(Box::new(Rock::new())),
                    _ => (),
                }
            }
        });

        for (index, i) in buffer.iter_mut().enumerate() {
            if let Some(p) = &model._particles[index] {
                *i = p.get_properties().color;
            } else {
                *i = 0;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        model.simulate();
    }
}
