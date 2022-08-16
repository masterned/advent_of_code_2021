pub fn calculate_rates(bit_weights: &[i32]) -> (i32, i32) {
    let (gamma_string, epsilon_string): (String, String) = bit_weights
        .iter()
        .map(|weight| {
            if weight.is_positive() {
                ('1', '0')
            } else {
                ('0', '1')
            }
        })
        .unzip();

    let gamma = i32::from_str_radix(&gamma_string, 2).expect("Not a binary string");
    let epsilon = i32::from_str_radix(&epsilon_string, 2).expect("Not a binary string");

    (gamma, epsilon)
}

pub fn calculate_power_consumption(diagnostic_report: &[String]) -> i32 {
    let mut bit_weights: Vec<i32> = vec![0; diagnostic_report[0].len()];

    for byte in diagnostic_report {
        for (index, bit) in byte.chars().enumerate() {
            match bit {
                '0' => {
                    bit_weights[index] -= 1;
                }
                '1' => {
                    bit_weights[index] += 1;
                }
                _ => {}
            }
        }
    }

    let (gamma_rate, epsilon_rate) = calculate_rates(&bit_weights);

    gamma_rate * epsilon_rate
}
