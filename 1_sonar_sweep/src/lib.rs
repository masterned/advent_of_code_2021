pub fn count_increases(depths: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previous_depth = depths[0];

    for depth in depths {
        if *depth > previous_depth {
            count += 1;
        }
        previous_depth = *depth;
    }

    count
}

pub fn count_window_mean_increases(depths: &[i32]) -> i32 {
    let mean_depths: Vec<i32> = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    count_increases(&mean_depths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, count_increases(&depths));
    }
}
