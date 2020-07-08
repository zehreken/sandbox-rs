use rand::prelude::*;
#[derive(Clone)]
pub struct ParticleProperties {
    pub kind: u8,
    pub color: u32,
    pub density: u8,
    // pub flammable: bool,
}

pub trait Particle {
    fn new() -> Self;
    fn simulate();
}

pub struct Sand {
    properties: ParticleProperties,
}

impl Particle for Sand {
    fn new() -> Sand {
        Sand {
            properties: ParticleProperties {
                kind: 0,
                color: 0xFFDEAD,
                density: 10,
            },
        }
    }
    fn simulate() {}
}

pub struct Water {
    properties: ParticleProperties,
}

impl Particle for Water {
    fn new() -> Water {
        Water {
            properties: ParticleProperties {
                kind: 1,
                color: 0x0EBFE9,
                density: 1,
            },
        }
    }
    fn simulate() {}
}

impl ParticleProperties {
    pub fn sand() -> Self {
        ParticleProperties {
            kind: 0,
            color: 0xFFDEAD,
            density: 10,
        }
    }

    pub fn water() -> Self {
        ParticleProperties {
            kind: 1,
            color: 0x0EBFE9,
            density: 1,
        }
    }
}

pub struct ParticleModel {
    pub width: usize,
    pub height: usize,
    pub particles: Vec<Option<ParticleProperties>>,
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
                                if let Some(left_p) = &self.particles[left] {
                                    if p.kind != 1 && left_p.kind == 1 {
                                        let temp = p.clone();
                                        self.particles[i] = Some(left_p.clone());
                                        self.particles[left] = Some(temp);
                                    }
                                } else {
                                    let temp = p.clone();
                                    self.particles[i] = None;
                                    self.particles[left] = Some(temp);
                                }
                            } else if let Some(right) =
                                get_index_right(below, self.width, self.height)
                            {
                                if let Some(right_p) = &self.particles[right] {
                                    if p.kind != 1 && right_p.kind == 1 {
                                        let temp = p.clone();
                                        self.particles[i] = Some(right_p.clone());
                                        self.particles[right] = Some(temp);
                                    }
                                } else {
                                    let temp = p.clone();
                                    self.particles[i] = None;
                                    self.particles[right] = Some(temp);
                                }
                            }
                        } else {
                            if let Some(right) = get_index_right(below, self.width, self.height) {
                                if let Some(right_p) = &self.particles[right] {
                                    if p.kind != 1 && right_p.kind == 1 {
                                        let temp = p.clone();
                                        self.particles[i] = Some(right_p.clone());
                                        self.particles[right] = Some(temp);
                                    }
                                } else {
                                    let temp = p.clone();
                                    self.particles[i] = None;
                                    self.particles[right] = Some(temp);
                                }
                            } else if let Some(left) =
                                get_index_right(below, self.width, self.height)
                            {
                                if let Some(left_p) = &self.particles[left] {
                                    if p.kind != 1 && left_p.kind == 1 {
                                        let temp = p.clone();
                                        self.particles[i] = Some(left_p.clone());
                                        self.particles[left] = Some(temp);
                                    }
                                } else {
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
