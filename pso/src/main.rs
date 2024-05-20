use rand;

#[derive(Debug)]
struct ParticleSwarm {
    position: Vec<f64>,
    velocity: Vec<f64>,
    local_optimum: Vec<f64>,
    global_optimum: Option<f64>,
}

impl ParticleSwarm {
    fn new(position: Vec<f64>, velocity: Vec<f64>) -> Self {
        Self {
            local_optimum: position.clone(),
            position,
            velocity,
            global_optimum: None,
        }
    }

    fn new_random(n: usize, f: fn(f64) -> f64, opt: &OptimizationPolicy) -> Self {
        let mut position = Vec::new();
        let mut velocity = Vec::new();
        let mut local_optimum = Vec::new();

        for _ in 0..n {
            let x = rand::random::<f64>();
            let v = rand::random::<f64>();
            position.push(x);
            velocity.push(v);
            local_optimum.push(x);
        }

        let global_optimum = local_optimum
            .iter()
            .max_by(|&x, &y| match opt {
                OptimizationPolicy::FindMinimum => f(*x).partial_cmp(&f(*y)).unwrap(),
                OptimizationPolicy::FindMaximum => f(*y).partial_cmp(&f(*x)).unwrap(),
            })
            .unwrap();

        Self {
            position,
            velocity,
            global_optimum: Some(*global_optimum),
            local_optimum,
        }
    }
}

struct UpdatePolicy {
    c1: f64,
    c2: f64,
}

impl UpdatePolicy {
    fn new(c1: f64, c2: f64) -> Self {
        Self { c1, c2 }
    }
}

enum OptimizationPolicy {
    FindMinimum,
    FindMaximum,
}

fn update(
    swarm: &mut ParticleSwarm,
    consts: &UpdatePolicy,
    f: fn(f64) -> f64,
    opt: &OptimizationPolicy,
) {
    // Update the particle's position
    for i in 0..swarm.position.len() {
        swarm.position[i] += swarm.velocity[i];
    }

    // Update the particle's best position
    for i in 0..swarm.position.len() {
        match opt {
            OptimizationPolicy::FindMinimum => {
                if f(swarm.position[i]) < f(swarm.local_optimum[i]) {
                    swarm.local_optimum[i] = swarm.position[i];
                }
            }
            OptimizationPolicy::FindMaximum => {
                if f(swarm.position[i]) > f(swarm.local_optimum[i]) {
                    swarm.local_optimum[i] = swarm.position[i];
                }
            }
        }
    }

    // Update the swarm's global best value
    let global_optimum = swarm
        .local_optimum
        .iter()
        .max_by(|&x, &y| match opt {
            OptimizationPolicy::FindMinimum => f(*y).partial_cmp(&f(*x)).unwrap(),
            OptimizationPolicy::FindMaximum => f(*x).partial_cmp(&f(*y)).unwrap(),
        })
        .unwrap();
    swarm.global_optimum = Some(*global_optimum);

    // Update the particle's velocity
    for i in 0..swarm.velocity.len() {
        let r1 = rand::random::<f64>();
        let r2 = rand::random::<f64>();
        swarm.velocity[i] = swarm.velocity[i]
            + consts.c1 * r1 * (swarm.local_optimum[i] - swarm.position[i])
            + consts.c2 * r2 * (swarm.global_optimum.unwrap() - swarm.position[i]);
    }
}

fn usage(program: &str) {
    println!("Usage: {} <n> <i>", program);
    println!("\tn: Number of particles");
    println!("\ti: Number of iterations");
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        usage(&args[0]);
        return;
    }
    let n = args[1].parse::<usize>().unwrap();
    let iter = args[2].parse::<usize>().unwrap();

    println!("Particle Swarm Optimization Demo");
    println!("Function to optimize: y = (x - 1)^2");

    let f = |x: f64| (x - 1.0) * (x - 1.0);
    let opt = OptimizationPolicy::FindMinimum;
    let consts = UpdatePolicy::new(0.5, 0.5);
    let mut swarm = ParticleSwarm::new_random(n, f, &opt);

    println!("Initialized {} particles:", n);
    // for i in 0..n {
    //     println!(
    //         "Particle {}: x = {}, y = {}",
    //         i,
    //         swarm.position[i],
    //         f(swarm.position[i])
    //     );
    // }
    // println!("{:#?}", swarm);

    for i in 0..iter {
        update(&mut swarm, &consts, f, &opt);
        // println!("Iteration {}: {:#?}", i, swarm);
        // println!("y = {}", f(swarm.global_optimum.unwrap()));
    }

    println!("Best value of x: {}", swarm.global_optimum.unwrap());
    println!("Best value of y: {}", f(swarm.global_optimum.unwrap()));
}
