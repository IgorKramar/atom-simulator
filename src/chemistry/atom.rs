use crate::physics::particle::Particle;

pub struct Atom {
    pub proton: Particle,
    pub electron: Particle,
    pub time: f64,
}

impl Atom {
    pub fn new() -> Self {
        let mut atom = Self {
            proton: Particle::new(1.6726e-27, 1.602e-19),
            electron: Particle::new(9.109e-31, -1.602e-19),
            time: 0.0,
        };

        atom.electron.position = [5.29177e-11, 0.0, 0.0];
        atom.electron.velocity = [0.0, 2.2e6, 0.0];
        atom
    }
}
