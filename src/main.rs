use graphics::math::{Vec2d, add, mul_scalar};

use piston::window::WindowSettings;
use piston_window::*;
use rand::{prelude::*, random_range};

const WINDOW_X: f64 = 1280.0;
const WINDOW_Y: f64 = 960.0;

pub struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

pub struct App {
    current_turn: u64,
    particles: Vec<Particle>,
    rng: ThreadRng,
}

pub struct Accumulation {
    stacks: [[f64; 2]; (WINDOW_X as usize) + 70],
}

impl Particle {
    fn new() -> Particle {
        let x = random_range(0.0..=WINDOW_X);
        let y = 0.0;
        let x_velocity = 0.0;
        let y_velocity = random_range(0.0..2.0);
        let x_acceleration = 0.0;
        let y_acceleration = random_range(0.0..0.15);

        Particle {
            height: 10.0,
            width: 10.0,
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        self.color[3] *= 0.995;
    }
}

impl App {
    fn update(&mut self) {
        let n = self.rng.random_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        self.current_turn += 1;
    }
    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new();
            self.particles.push(particle);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;

            let particle_iter = self.particles.iter().enumerate();

            for (i, particle) in particle_iter {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }
            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                if self.particles.len() > 0 {
                    self.particles.remove(0);
                }
            };
        }
    }
}

impl Accumulation {
    fn new() -> Accumulation {
        let mut stacks = [[0.0; 2]; (WINDOW_X as usize) + 70];
        for i in 0..stacks.len() {
            stacks[i][0] = i as f64;
            stacks[i][1] = WINDOW_Y + 200.0;
        }
        Accumulation { stacks }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snowfall", [WINDOW_X, WINDOW_Y])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window.");

    let mut world = App {
        current_turn: 0,
        particles: Vec::<Particle>::new(), // <19>
        rng: rand::rng(),
    };
    world.add_shapes(1000);

    let mut snow_drift = Accumulation::new();

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
                let wiggle = world.rng.random_range(0.0..30.0); // added to keep from seeing overly
                // jagged mounds that match the not so random distribution
                //
                // Trying to make it seem a bit more random... I need to do some thinking here
                for i in 0..30 {
                    snow_drift.stacks[(s.position[0] + wiggle + (i as f64)) as usize][1] -= 0.05;
                }
            }
        });
        window.draw_2d(&event, |ctx, renderer, _device| {
            polygon(
                [1.0, 1.0, 1.0, 1.0],
                &snow_drift.stacks,
                ctx.transform,
                renderer,
            );
        });
    }
}
