use std::io;

use input_reader::read_lines;

fn main() -> io::Result<()> {
    let lines = read_lines("3_binary_diagnostic/data/input.txt")?;

    for line in lines {
        println!("{line}");
    }

    Ok(())
}
