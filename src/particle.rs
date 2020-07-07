#[derive(Clone)]
pub struct Particle {
    pub color: u32,
    pub density: u8,
    // pub flammable: bool,
}

impl Particle {
    pub fn default() -> Self {
        Particle {
            color: 0xFFDEAD,
            density: 0,
        }
    }
}

pub struct ParticleModel {
    pub particles: Vec<Option<Particle>>,
}

impl ParticleModel {
    pub fn new(width: usize, height: usize) -> Self {
        ParticleModel {
            particles: vec![None; width * height],
        }
    }
}
