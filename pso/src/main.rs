use colored::Colorize;
use rand;
use std::fmt;

#[derive(Debug)]
struct ParticleSwarm {
    position: Vec<f64>,
    velocity: Vec<f64>,
    local_optimum: Vec<f64>,
    global_optimum: Option<f64>,
}

impl ParticleSwarm {
    fn new(n: usize, x: Vec<f64>, v: Vec<f64>, f: fn(f64) -> f64, opt: &OptimizationPolicy) -> Self {
        assert!(x.len() == n, "Position vector must have length equal to number of particles");
        assert!(v.len() == n, "Velocity vector must have length equal to number of particles");

        let mut local_optimum = Vec::new();
        for i in 0..n {
            local_optimum.push(x[i]);
        }

        let global_optimum = local_optimum
            .iter()
            .max_by(|&x, &y| match opt {
                OptimizationPolicy::FindMinimum => f(*x).partial_cmp(&f(*y)).unwrap(),
                OptimizationPolicy::FindMaximum => f(*y).partial_cmp(&f(*x)).unwrap(),
            })
            .unwrap();

        Self {
            position: x,
            velocity: v,
            global_optimum: Some(*global_optimum),
            local_optimum,
        }
    }
    fn new_random<R: rand::Rng>(
        n: usize,
        f: fn(f64) -> f64,
        opt: &OptimizationPolicy,
        r: &mut R,
    ) -> Self {
        let mut position = Vec::new();
        let mut velocity = Vec::new();
        let mut local_optimum = Vec::new();

        for _ in 0..n {
            let x: f64 = r.gen();
            let v: f64 = r.gen();
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

impl fmt::Display for ParticleSwarm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
"Positions: {:?}
Velocities: {:?}",
            self.position, self.velocity
        )
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

fn update<R: rand::Rng>(
    swarm: &mut ParticleSwarm,
    consts: &UpdatePolicy,
    f: fn(f64) -> f64,
    opt: &OptimizationPolicy,
    r: &mut R,
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
        let r1 = r.gen::<f64>();
        let r2 = r.gen::<f64>();
        swarm.velocity[i] = swarm.velocity[i]
            + consts.c1 * r1 * (swarm.local_optimum[i] - swarm.position[i])
            + consts.c2 * r2 * (swarm.global_optimum.unwrap() - swarm.position[i]);
    }
}

fn usage(program: &str) {
    println!(
        "Usage: {} -n <n> (-e <e>|-i <i>) [-v] [--seed <seed>] [--init <x1,x2,...,xn>] [--vinit <v1,v2,...,vn>]",
        program
    );
    println!("\t-n: Number of particles\t(required)");
    println!("\t-e: Error threshold\t(default:0.0001)");
    println!("\t-i: Number of iterations\t(uses error threshold if not provided)");
    println!("\t-v: Verbose mode\t(default:false)");
    println!("\t--seed: Use a fixed seed for random number generation");
    println!("\t--init: Initial positions of particles");
    println!("\t--vinit: Initial velocities of particles");
}

enum ParseError {
    MissingArgument(String),
    InvalidParticleNumber(String),
    InvalidIterations(String),
    InvalidThreshold(String),
    InvalidSeed(String),
    InvalidArgument(String),
}

struct RunOptions {
    n: usize,
    iter: Option<usize>,
    thresh: f64,
    verbose: bool,
    init: Option<Vec<f64>>,
    vinit: Option<Vec<f64>>,
    r: Option<rand::rngs::StdRng>,
}

fn parse(args: &Vec<String>) -> Result<RunOptions, ParseError> {
    let mut n = None;
    let mut iter = None;
    let mut verbose = false;
    let mut thresh = 0.0001;
    let mut r = None;

    let mut init = None;
    let mut vinit = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-n" => {
                if i + 1 >= args.len() {
                    return Err(ParseError::MissingArgument("-n".to_string()));
                }
                n = Some(
                    args[i + 1]
                        .parse::<usize>()
                        .map_err(|_| ParseError::InvalidParticleNumber(args[i + 1].clone()))?,
                );
                i += 2;
            }
            "-i" => {
                if i + 1 >= args.len() {
                    return Err(ParseError::MissingArgument("-i".to_string()));
                }
                iter = Some(
                    args[i + 1]
                        .parse::<usize>()
                        .map_err(|_| ParseError::InvalidIterations(args[i + 1].clone()))?,
                );
                i += 2;
            }
            "-e" => {
                if i + 1 >= args.len() {
                    return Err(ParseError::MissingArgument("-e".to_string()));
                }
                thresh = args[i + 1]
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidThreshold(args[i + 1].clone()))?;
                i += 2;
            }
            "-v" => {
                verbose = true;
                i += 1;
            }
            "--seed" => {
                if i + 1 >= args.len() {
                    return Err(ParseError::MissingArgument("--seed".to_string()));
                }
                let seed = args[i + 1]
                    .parse::<u64>()
                    .map_err(|_| ParseError::InvalidSeed(args[i + 1].clone()))?;
                println!("Using seed {}", seed);
                r = Some(rand::SeedableRng::seed_from_u64(seed));
                i += 2;
            }
            "--init" => {
                if i + 1 >= args.len() {
                    return Err(ParseError::MissingArgument("--init".to_string()));
                }
                init = Some(
                    args[i + 1]
                        .split(",")
                        .map(|x| {
                            x.parse::<f64>()
                                .map_err(|_| ParseError::InvalidArgument(x.to_string()))
                        })
                        .collect::<Result<Vec<f64>, ParseError>>()?,
                );
                i += 2;
            }
            "--vinit" => {
                if i + 1 >= args.len() {
                    return Err(ParseError::MissingArgument("--vinit".to_string()));
                }
                vinit = Some(
                    args[i + 1]
                        .split(",")
                        .map(|x| {
                            x.parse::<f64>()
                                .map_err(|_| ParseError::InvalidArgument(x.to_string()))
                        })
                        .collect::<Result<Vec<f64>, ParseError>>()?,
                );
                i += 2;
            }
            _ => {
                return Err(ParseError::InvalidArgument(args[i].clone()));
            }
        }
    }

    Ok(RunOptions {
        n: n.ok_or(ParseError::MissingArgument("-n".to_string()))?,
        iter,
        thresh,
        verbose,
        init,
        vinit,
        r,
    })
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let run_opts = match parse(&args) {
        Ok(opts) => opts,
        Err(ParseError::MissingArgument(arg)) => {
            eprintln!("Missing argument for {}", arg.red());
            usage(&args[0]);
            std::process::exit(1);
        }
        Err(ParseError::InvalidParticleNumber(arg)) => {
            eprintln!("Invalid number of particles: {}", arg.red());
            usage(&args[0]);
            std::process::exit(1);
        }
        Err(ParseError::InvalidIterations(arg)) => {
            eprintln!("Invalid number of iterations: {}", arg.red());
            usage(&args[0]);
            std::process::exit(1);
        }
        Err(ParseError::InvalidThreshold(arg)) => {
            eprintln!("Invalid error threhold: {}", arg.red());
            usage(&args[0]);
            std::process::exit(1);
        }
        Err(ParseError::InvalidSeed(arg)) => {
            eprintln!("Invalid seed: {}", arg.red());
            usage(&args[0]);
            std::process::exit(1);
        }
        Err(ParseError::InvalidArgument(arg)) => {
            eprintln!("Unexpected argument: {}", arg.red());
            usage(&args[0]);
            std::process::exit(1);
        }
    };

    let n = run_opts.n;
    let iter = run_opts.iter;
    let thresh = run_opts.thresh;
    let verbose = run_opts.verbose;
    let mut r = match run_opts.r {
        Some(rng) => rng,
        None => {
            println!("Using random seed");
            rand::SeedableRng::from_entropy()
        }
    };

    println!("Particle Swarm Optimization Demo");
    println!("Function to optimize: y = (x - 1)^2");

    let f = |x: f64| (x - 1.0) * (x - 1.0);
    let opt = OptimizationPolicy::FindMinimum;
    let consts = UpdatePolicy::new(0.5, 0.5);
    let mut swarm = match run_opts.init {
        Some(x) => match run_opts.vinit {
            Some(v) => ParticleSwarm::new(n, x, v, f, &opt),
            None => ParticleSwarm::new(n, x, vec![0.0; n], f, &opt),
        },
        None => ParticleSwarm::new_random(n, f, &opt, &mut r),
    };

    println!("\nInitialized {} particles:", n);
    if verbose {
        println!("{}\n", swarm);
    }
    match iter {
        Some(i) => {
            for _ in 1..i+1 {
                update(&mut swarm, &consts, f, &opt, &mut r);
                if verbose {
                    println!("Iteration {}", i);
                    println!("{}\n", swarm);
                }
            }
        }
        None => {
            let mut i = 1;
            while f(swarm.global_optimum.unwrap()) > thresh {
                update(&mut swarm, &consts, f, &opt, &mut r);
                if verbose {
                    println!("Iteration {}", i);
                    println!("{}\n", swarm);
                }
                i += 1;
            }
            println!("Finished in {} iterations", i);
        }
    }

    println!("Best value of x: {}", swarm.global_optimum.unwrap());
    println!("Best value of y: {}", f(swarm.global_optimum.unwrap()));
}
