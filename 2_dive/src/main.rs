use std::io;

use dive::{Command, process_commands_basic, process_aim_commands};
use input_reader::read_lines;

fn main() -> io::Result<()> {
    let lines = read_lines("2_dive/data/input.txt")?;

    let commands: Vec<Command> = lines.iter().filter_map(|line| line.parse::<Command>().ok()).collect();

    let answer = process_commands_basic(&commands);

    println!("Part 1: {answer}");

    let answer = process_aim_commands(&commands);

    println!("Part 2: {answer}");

    Ok(())
}
