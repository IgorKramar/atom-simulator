use super::atom::Atom;
use crate::physics::{forces, integrators, particle::Particle};

pub struct Simulation {
    pub atoms: [Atom; 2],
    pub time: f64,
}

impl Simulation {
    pub fn new() -> Self {
        let mut atom1 = Atom::new();
        let mut atom2 = Atom::new();

        // Начальные условия для электронов
        atom1.electron.velocity = [0.0, 5.0e4, 0.0]; // Уменьшили скорость ещё в 3 раза
        atom2.electron.velocity = [0.0, -5.0e4, 0.0];

        // Начальные условия для столкновения
        atom1.proton.position = [-1e-10, 0.0, 0.0];
        atom1.electron.position = [-1e-10 - 5.29177e-11, 0.0, 0.0];
        atom1.electron.velocity = [1e6, 0.0, 0.0];

        atom2.proton.position = [1e-10, 0.0, 0.0];
        atom2.electron.position = [1e-10 + 5.29177e-11, 0.0, 0.0];
        atom2.electron.velocity = [-1e6, 0.0, 0.0];

        Self {
            atoms: [atom1, atom2],
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut particles = self.all_particles_mut();

        // Рассчитываем все взаимодействия
        for i in 0..particles.len() {
            let (head, tail) = particles.split_at_mut(i + 1);
            let particle_i = &mut head[i];

            for particle_j in tail {
                let force = forces::coulomb_force(particle_i, particle_j);

                // Обновляем скорости для обеих частиц
                let accel_i = [
                    force[0] / particle_i.mass,
                    force[1] / particle_i.mass,
                    force[2] / particle_i.mass,
                ];

                let accel_j = [
                    -force[0] / particle_j.mass,
                    -force[1] / particle_j.mass,
                    -force[2] / particle_j.mass,
                ];

                particle_i.velocity[0] += accel_i[0] * dt;
                particle_i.velocity[1] += accel_i[1] * dt;
                particle_i.velocity[2] += accel_i[2] * dt;

                particle_j.velocity[0] += accel_j[0] * dt;
                particle_j.velocity[1] += accel_j[1] * dt;
                particle_j.velocity[2] += accel_j[2] * dt;
            }
        }

        // Интегрируем движение
        for particle in particles {
            integrators::verlet_integrate(particle, dt);
        }

        self.time += dt;

        // Проверка столкновений
        let distance = ((self.atoms[0].proton.position[0] - self.atoms[1].proton.position[0])
            .powi(2)
            + (self.atoms[0].proton.position[1] - self.atoms[1].proton.position[1]).powi(2))
        .sqrt();

        if distance < 5e-11 {
            // Порог столкновения
            self.handle_collision();
        }
    }

    fn handle_collision(&mut self) {
        // Простая модель упругого столкновения
        let vx1 = self.atoms[0].proton.velocity[0];
        let vx2 = self.atoms[1].proton.velocity[0];

        self.atoms[0].proton.velocity[0] = vx2;
        self.atoms[1].proton.velocity[0] = vx1;

        // Добавим визуальный эффект
        self.atoms[0].proton.velocity[1] += 1e5;
        self.atoms[1].proton.velocity[1] -= 1e5;
    }

    fn all_particles_mut(&mut self) -> Vec<&mut Particle> {
        self.atoms
            .iter_mut()
            .flat_map(|atom| vec![&mut atom.proton, &mut atom.electron])
            .collect()
    }

    pub fn print_state(&self) {
        println!("Время: {:.2e} с", self.time);
        for (i, atom) in self.atoms.iter().enumerate() {
            println!("Атом {}:", i + 1);
            println!("  Протон: {:?}", atom.proton.position);
            println!("  Электрон: {:?}", atom.electron.position);
        }
        println!("----------------------------------------");
        println!(
            "Расстояние между атомами: {:.2} Å",
            ((self.atoms[0].proton.position[0] - self.atoms[1].proton.position[0]).abs() * 1e10)
        );
    }
}
