use std::io;

use binary_diagnostic::calculate_power_consumption;
use input_reader::read_lines;

fn main() -> io::Result<()> {
    let lines = read_lines("3_binary_diagnostic/data/input.txt")?;

    match calculate_power_consumption(&lines) {
        Ok(power_consumption) => {
            println!("Part 1: {power_consumption}");
        }
        Err(diag_error) => {
            panic!("{diag_error}");
        }
    }

    Ok(())
}
