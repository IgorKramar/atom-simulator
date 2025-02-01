use super::particle::Particle;

pub fn coulomb_force(particle1: &Particle, particle2: &Particle) -> [f64; 3] {
    let k = 8.9875e9; // Постоянная Кулона
    let r = [
        particle1.position[0] - particle2.position[0],
        particle1.position[1] - particle2.position[1],
        particle1.position[2] - particle2.position[2],
    ];

    let distance = (r[0].powi(2) + r[1].powi(2) + r[2].powi(2)).sqrt();
    let force_magnitude = k * particle1.charge * particle2.charge / distance.powi(2);

    [
        -force_magnitude * r[0] / distance,
        -force_magnitude * r[1] / distance,
        -force_magnitude * r[2] / distance,
    ]
} 