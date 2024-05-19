use rand::Rng;
use std::env;

fn main() {

    let mut rng = rand::thread_rng();
    let mut inside_circle = 0;

    let iterations: Result<i32, _> = env::var("ITER")
        .map_err(|e| e.to_string())  // Convert the error to a string if there is an error
        .and_then(|v| v.parse::<i32>().map_err(|e| e.to_string()));  // Convert the parse error to a string if there is an error

    let iterations = match iterations.clone() {
        Ok(num) => {
            println!("ITER is: {}", num);
            num
        }
        Err(e) => {
            println!("Error retrieving ITER as integer: {}", e);
            return;
        }
    };

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
    let pi = 4.0 * (inside_circle as f64) / (iterations as f64);
    println!("π is roughly {}", pi);
}
