use super::particle::Particle;

pub fn verlet_integrate(particle: &mut Particle, dt: f64) {
    particle.position[0] += particle.velocity[0] * dt + 0.5 * particle.acceleration[0] * dt.powi(2);
    particle.position[1] += particle.velocity[1] * dt + 0.5 * particle.acceleration[1] * dt.powi(2);
    particle.position[2] += particle.velocity[2] * dt + 0.5 * particle.acceleration[2] * dt.powi(2);

    // Сбрасываем ускорение после интегрирования
    particle.acceleration = [0.0; 3];
} 