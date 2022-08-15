#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::{fs, io};

/// Reads the file at the given path and returns its contents, line-by-line as a vector of 32-bit
/// signed integers.
///
/// # Errors
///
/// If something goes wrong while trying to read the file -- file does not exist, user does not
/// have read access to the file, etc -- this function will exit early, returning the resulting
/// file `Error`.
pub fn read_lines_as_vec_i32(path: &str) -> io::Result<Vec<i32>> {
    let contents = fs::read_to_string(path)?;

    let values = contents
        .trim()
        .split('\n')
        .map(String::from)
        .filter_map(|data| data.parse::<i32>().ok())
        .collect();

    Ok(values)
}

/// Reads the file at the given path and returns its contents separated by line.
///
/// # Errors
///
/// If something goes wrong while trying to read the file -- file does not exist, user does not
/// have access to the file, etc -- this function will exit early, returning the resulting file
/// `Error`.
pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(path)?;

    let values = contents.trim().split('\n').map(String::from).collect();

    Ok(values)
}
