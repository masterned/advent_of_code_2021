#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::io;

use input_reader::read_lines_as_vec_i32;
use sonar_sweep::{count_increases, count_window_mean_increases};

fn main() -> io::Result<()> {
    let depths = read_lines_as_vec_i32("1_sonar_sweep/data/puzzle_input.txt")?;

    let increase_count = count_increases(&depths);

    println!("Part 1: {increase_count}");

    let increase_count = count_window_mean_increases(&depths);

    println!("Part 2: {increase_count}");

    Ok(())
}
