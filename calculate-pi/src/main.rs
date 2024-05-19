use rand::Rng;
use std::env;

fn main() {

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut inside_circle = 0;

    let iter_var_result: Result<String, env::VarError> = env::var("ITER");

    // Handle the case where the environment variable is not found or cannot be parsed as i32
    let iterations: i32 = match iter_var_result {
        Ok(iter_str) => match iter_str.parse::<i32>() {
            Ok(iterations) => {
                println!("ITER is: {}", iterations);
                iterations
            },
            Err(parse_error) => {
                println!("Error parsing ITER as i32: {}", parse_error);
                100000 // Return a default value
            },
        },
        Err(_) => {
            println!("Environment variable ITER not found.");
            100000 // Return a default value
        },
    };

    // Use iterations here
    println!("Final value of iterations: {}", iterations);

    let start: std::time::Instant = std::time::Instant::now();
    for _ in 0..iterations {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0..1.0);
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    let duration: std::time::Duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
    let pi: f64 = 4.0 * (inside_circle as f64) / (iterations as f64);
    println!("π is roughly {}", pi);
}
