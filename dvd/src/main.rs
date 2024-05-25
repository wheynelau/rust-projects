use rand::Rng;
use std::{thread, time};
use ndarray::Array2;
use colored::Colorize;


enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    // Generates a random RGB color
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Color::new(rng.gen(), rng.gen(), rng.gen())
    }
    fn print(&self) {
        println!("Color: {:?}", self);
    }
}


fn convert_lines_to_2d_array(lines: Vec<&str>) -> Result<Array2<char>, Box<dyn std::error::Error>> {
    // Find the maximum line length
    let max_line_length = lines.iter()
                                .max_by_key(|line| line.len())
                                .ok_or("No lines")?.len();

    // Collect each character from each line, padding shorter lines with spaces
    let rows: Vec<Vec<char>> = lines.iter().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        let padding_needed = max_line_length - chars.len();
        let padded_chars = chars.into_iter().chain(std::iter::repeat(' ').take(padding_needed));
        padded_chars.collect()
    }).collect();

    let n_rows = rows.len();
    let n_cols = max_line_length;
    let flat_vec: Vec<char> = rows.into_iter().flatten().collect();

    Ok(Array2::from_shape_vec((n_rows, n_cols), flat_vec)?)
}
fn main() {

    let width = 100;
    let height = 30;

    let mut color = Color::random();

    let sleep_time = time::Duration::from_secs_f64(0.1);

    // Random starting position
    let mut x = rand::thread_rng().gen_range(0..width /2);
    let mut y = rand::thread_rng().gen_range(0..height / 2);

    let dvd: Vec<&str> = vec![
    "⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣶⣦⡀",
    "⠀⢠⣿⣿⡿⠀⠀⠈⢹⣿⣿⡿⣿⣿⣇⠀⣠⣿⣿⠟⣽⣿⣿⠇⠀⠀⢹⣿⣿⣿",
    "⠀⢸⣿⣿⡇⠀⢀⣠⣾⣿⡿⠃⢹⣿⣿⣶⣿⡿⠋⢰⣿⣿⡿⠀⠀⣠⣼⣿⣿⠏",
    "⠀⣿⣿⣿⣿⣿⣿⠿⠟⠋⠁⠀⠀⢿⣿⣿⠏⠀⠀⢸⣿⣿⣿⣿⣿⡿⠟⠋⠁⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣸⣟⣁⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    "⣠⣴⣶⣾⣿⣿⣻⡟⣻⣿⢻⣿⡟⣛⢻⣿⡟⣛⣿⡿⣛⣛⢻⣿⣿⣶⣦⣄⡀",
    "⠉⠛⠻⠿⠿⠿⠷⣼⣿⣿⣼⣿⣧⣭⣼⣿⣧⣭⣿⣿⣬⡭⠾⠿⠿⠿⠛⠉⠀ "];

    let dvd_matrix = convert_lines_to_2d_array(dvd).unwrap();

    let mut x_dir = Direction::Right;
    let mut y_dir = Direction::Down;

    // println!("DVD logo starting at: {:?}", (x, y));

    let (dim_x, dim_y) = dvd_matrix.dim();

    loop {
        println!("DVD logo starting at: {:?}", (x, y));
        for i in 0..height {
            for j in 0..width {
                if i >= y && i < y + dim_x && j >= x && j < x + dim_y {
                    print!("{}", dvd_matrix[[i - y, j - x]]
                    .to_string()
                    .truecolor(color.r, color.g, color.b));
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        match x_dir {
            Direction::Right => {
                if x + 1 < width - 50 {
                    x += 1;
                } else {
                    x_dir = Direction::Left;
                    color = Color::random();
                }
            }
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else {
                    x_dir = Direction::Right;
                    color = Color::random();
                }
            }
            _ => {}
        }

        match y_dir {
            Direction::Down => {
                if y + 1 < height - dim_x {
                    y += 1;
                } else {
                    y_dir = Direction::Up;
                    color = Color::random();
                }
            }
            Direction::Up => {
                if y > 0 {
                    y -= 1;
                } else {
                    y_dir = Direction::Down;
                    color = Color::random();
                }
            }
            _ => {}
        }
        thread::sleep(sleep_time);
        clearscreen::clear().unwrap();

    }

}
