fn calculate_bit_weights(diagnostic_report: &[String]) -> Vec<i32> {
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
            };
        }
    }

    bit_weights
}

fn convert_bit_weight_to_binary_string(bit_weights: &[i32]) -> String {
    bit_weights
        .iter()
        .map(|&weight| if weight.is_positive() { '1' } else { '0' })
        .collect()
}

fn invert_binary_string(binary_string: &str) -> String {
    binary_string
        .chars()
        .map(|bit| match bit {
            '0' => '1',
            '1' => '0',
            _ => bit,
        })
        .collect()
}

pub fn calculate_power_consumption(diagnostic_report: &[String]) -> i32 {
    let bit_weights = calculate_bit_weights(diagnostic_report);
    let gamma_string = convert_bit_weight_to_binary_string(&bit_weights);
    let epsilon_string = invert_binary_string(&gamma_string);
    let gamma_rate = i32::from_str_radix(&gamma_string, 2).expect("Not a binary string");
    let epsilon_rate = i32::from_str_radix(&epsilon_string, 2).expect("Not a binary string");

    gamma_rate * epsilon_rate
}
