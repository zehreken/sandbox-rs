#[derive(Clone)]
pub struct Particle {
    pub color: u32,
    pub density: u8,
    // pub flammable: bool,
}

impl Particle {
    pub fn sand() -> Self {
        Particle {
            color: 0xFFDEAD,
            density: 10,
        }
    }

    pub fn water() -> Self {
        Particle {
            color: 0x0EBFE9,
            density: 1,
        }
    }
}

pub struct ParticleModel {
    pub width: usize,
    pub height: usize,
    pub particles: Vec<Option<Particle>>,
}

impl ParticleModel {
    pub fn new(width: usize, height: usize) -> Self {
        ParticleModel {
            width,
            height,
            particles: vec![None; width * height],
        }
    }

    pub fn simulate(&mut self) {
        for i in (0..self.particles.len()).rev() {
            if let Some(p) = &self.particles[i] {
                if let Some(below) = get_index_below(i, self.width, self.height) {
                    if let Some(below_p) = &self.particles[below] {
                        if below_p.density < p.density {
                            let temp = p.clone();
                            self.particles[i] = Some(below_p.clone());
                            self.particles[below] = Some(temp);
                        }
                    } else {
                        let temp = p.clone();
                        self.particles[i] = None;
                        self.particles[below] = Some(temp);
                    }
                }
            }
        }
    }
}

fn get_index_below(index: usize, width: usize, height: usize) -> Option<usize> {
    let index = index + width;
    if index < width * height {
        Some(index)
    } else {
        None
    }
}
