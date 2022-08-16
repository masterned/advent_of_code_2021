use std::io;

use binary_diagnostic::calculate_power_consumption;
use input_reader::read_lines;

fn main() -> io::Result<()> {
    let lines = read_lines("3_binary_diagnostic/data/input.txt")?;

    let power_consumption = calculate_power_consumption(&lines);

    println!("Part 1: {power_consumption}");

    Ok(())
}
