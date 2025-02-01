#[derive(Debug, Clone)]
pub struct Particle {
    pub mass: f64,
    pub charge: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub acceleration: [f64; 3],
}

impl Particle {
    pub fn new(mass: f64, charge: f64) -> Self {
        Self {
            mass,
            charge,
            position: [0.0; 3],
            velocity: [0.0; 3],
            acceleration: [0.0; 3],
        }
    }
}
