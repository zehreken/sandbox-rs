use rand::prelude::*;
type SimulationResult = (
    (usize, Option<Box<dyn Particle>>),
    (usize, Option<Box<dyn Particle>>),
);
#[derive(Clone)]
pub struct ParticleProperties {
    pub kind: u8,
    pub color: u32,
    pub density: u8,
    // pub flammable: bool,
}

pub trait Particle: ParticleClone {
    // POST: Explain why you have 'where Self: Sized'
    fn new() -> Self
    where
        Self: Sized;
    fn get_properties(&self) -> &ParticleProperties;
    fn simulate(
        &self,
        index: usize,
        particles: &Vec<Option<Box<dyn Particle>>>,
    ) -> Option<SimulationResult>;
}

// POST: Explain clone implemention for traits
pub trait ParticleClone {
    fn clone_box(&self) -> Box<dyn Particle>;
}

impl<T: 'static + Particle + Clone> ParticleClone for T {
    fn clone_box(&self) -> Box<dyn Particle> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Particle> {
    fn clone(&self) -> Box<dyn Particle> {
        self.clone_box()
    }
}

#[derive(Clone)]
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

    fn get_properties(&self) -> &ParticleProperties {
        &self.properties
    }

    fn simulate(
        &self,
        index: usize,
        particles: &Vec<Option<Box<dyn Particle>>>,
    ) -> Option<SimulationResult> {
        let mut result = None;
        if let Some(below) = get_index_down(index, super::WIDTH, super::HEIGHT) {
            if let Some(_) = &particles[below] {
                if let Some(left_to_below) = get_index_left(below, super::WIDTH, super::HEIGHT) {
                    if let None = &particles[left_to_below] {
                        let temp = self.clone();
                        let sim_res: SimulationResult =
                            ((index, None), (left_to_below, Some(Box::new(temp))));
                        result = Some(sim_res);
                    }
                }
                if let Some(right_to_below) = get_index_right(below, super::WIDTH, super::HEIGHT) {
                    if let None = &particles[right_to_below] {
                        let temp = self.clone();
                        let sim_res: SimulationResult =
                            ((index, None), (right_to_below, Some(Box::new(temp))));
                        result = Some(sim_res);
                    }
                }
            } else {
                // If there is nothing below, just fall
                let temp = self.clone();
                let sim_res: SimulationResult = ((index, None), (below, Some(Box::new(temp))));
                result = Some(sim_res);
            }
        }

        result
    }
}

#[derive(Clone)]
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

    fn get_properties(&self) -> &ParticleProperties {
        &self.properties
    }

    fn simulate(
        &self,
        index: usize,
        particles: &Vec<Option<Box<dyn Particle>>>,
    ) -> Option<SimulationResult> {
        let mut result = None;
        if let Some(below) = get_index_down(index, super::WIDTH, super::HEIGHT) {
            let mut moved = false;
            if let Some(_) = &particles[below] {
                if let Some(left) = get_index_left(index, super::WIDTH, super::HEIGHT) {
                    if let None = &particles[left] {
                        let temp = self.clone();
                        let sim_res: SimulationResult =
                            ((index, None), (left, Some(Box::new(temp))));
                        result = Some(sim_res);
                        moved = true;
                    }
                }
                if !moved {
                    if let Some(right) = get_index_right(index, super::WIDTH, super::HEIGHT) {
                        if let None = &particles[right] {
                            let temp = self.clone();
                            let sim_res: SimulationResult =
                                ((index, None), (right, Some(Box::new(temp))));
                            result = Some(sim_res);
                        }
                    }
                }
            } else {
                // If there is nothing below, just fall
                let temp = self.clone();
                let sim_res: SimulationResult = ((index, None), (below, Some(Box::new(temp))));
                result = Some(sim_res);
            }
        }

        result
    }
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
    pub _particles: Vec<Option<Box<dyn Particle>>>,
}

impl ParticleModel {
    pub fn new(width: usize, height: usize) -> Self {
        ParticleModel {
            width,
            height,
            particles: vec![None; width * height],
            _particles: vec![None; width * height],
        }
    }

    pub fn _simulate(&mut self) {
        // Simulate from bottom to top
        for i in (0..self._particles.len()).rev() {
            if let Some(p) = &self._particles[i] {
                let result = p.simulate(i, &self._particles);
                if let Some(r) = result {
                    self._particles[(r.0).0] = (r.0).1;
                    self._particles[(r.1).0] = (r.1).1;
                }
            }
        }
    }

    #[deprecated = "There is a new implementation"]
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
