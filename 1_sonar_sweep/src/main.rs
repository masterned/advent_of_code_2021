#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::io;

use input_reader::read_lines_as_vec_i32;
use sonar_sweep::count_increases;

fn main() -> io::Result<()> {
    let depths = read_lines_as_vec_i32("1_sonar_sweep/data/puzzle_input.txt")?;

    let increase_count = count_increases(&depths);

    println!("{increase_count}");

    Ok(())
}
