use std::io;

use dive::Command;
use input_reader::read_lines;

fn main() -> io::Result<()> {
    let lines = read_lines("2_dive/data/input.txt")?;

    let mut x = 0;
    let mut y = 0;

    for line in lines {
        if let Ok(command) = line.parse::<Command>() {
            match command {
                Command::Forward(amount) => {
                    x += amount;
                }
                Command::Up(amount) => {
                    y -= amount;
                }
                Command::Down(amount) => {
                    y += amount;
                }
            }
        }
    }

    println!("Part 1: {}", x * y);

    Ok(())
}
