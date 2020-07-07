use rand::prelude::*;
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
        let mut rng = rand::thread_rng();
        for i in (0..self.particles.len()).rev() {
            if let Some(p) = &self.particles[i] {
                if let Some(below) = get_index_down(i, self.width, self.height) {
                    if let Some(below_p) = &self.particles[below] {
                        let is_left = rng.gen::<f32>() < 0.5;
                        // If there is something below
                        if below_p.density < p.density {
                            let temp = p.clone();
                            self.particles[i] = Some(below_p.clone());
                            self.particles[below] = Some(temp);
                        } else if is_left {
                            if let Some(left) = get_index_left(below, self.width, self.height) {
                                if let None = &self.particles[left] {
                                    let temp = p.clone();
                                    self.particles[i] = None;
                                    self.particles[left] = Some(temp);
                                }
                            } else if let Some(right) =
                                get_index_right(below, self.width, self.height)
                            {
                                if let None = &self.particles[right] {
                                    let temp = p.clone();
                                    self.particles[i] = None;
                                    self.particles[right] = Some(temp);
                                }
                            }
                        } else {
                            if let Some(right) = get_index_right(below, self.width, self.height) {
                                if let None = &self.particles[right] {
                                    let temp = p.clone();
                                    self.particles[i] = None;
                                    self.particles[right] = Some(temp);
                                }
                            } else if let Some(left) =
                                get_index_right(below, self.width, self.height)
                            {
                                if let None = &self.particles[left] {
                                    let temp = p.clone();
                                    self.particles[i] = None;
                                    self.particles[left] = Some(temp);
                                }
                            }
                        }
                    } else {
                        // If there is nothing below, just fall
                        let temp = p.clone();
                        self.particles[i] = None;
                        self.particles[below] = Some(temp);
                    }
                }
            }
        }
    }
}

fn get_index_down(index: usize, width: usize, height: usize) -> Option<usize> {
    let index = index + width;
    if index < width * height {
        Some(index)
    } else {
        None
    }
}

fn get_index_left(index: usize, width: usize, height: usize) -> Option<usize> {
    let index = index - 1;
    if index < width * height {
        Some(index)
    } else {
        None
    }
}

fn get_index_right(index: usize, width: usize, height: usize) -> Option<usize> {
    let index = index + 1;
    if index < width * height {
        Some(index)
    } else {
        None
    }
}
