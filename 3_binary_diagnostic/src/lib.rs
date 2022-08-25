use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DiagnosticError {
    /// Occurs when trying to parse a byte in the diagnostic report that is not encoded in binary.
    ParseBinaryError,
    /// Occurs when the gamma & epsilon rate values cause an integer overflow.
    OverflowError,
    /// Occurs when byte's length does not match the rest of the diagnostic report.
    ByteLengthError,
    /// This error should not occur ever. If it does, something is very wrong...
    ImpossibleStateError,
    /// Occurs when the diagnostic report is empty
    EmptyReportError,
}

impl fmt::Display for DiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OverflowError => {
                write!(
                    f,
                    "Magnitude of gamma & epsilon rates caused an integer overflow."
                )
            }
            Self::ParseBinaryError => {
                write!(f, "Attempted to parse a non-binary string as byte.")
            }
            Self::ByteLengthError => {
                write!(f, "Byte length incongruent with other byte(s).")
            }
            Self::ImpossibleStateError => {
                write!(f, "Somehow reached an impossible state.")
            }
            Self::EmptyReportError => {
                write!(f, "The provied diagnostic report was empty.")
            }
        }
    }
}

impl Error for DiagnosticError {}

fn calculate_bit_weights(diagnostic_report: &[String]) -> Result<Vec<i32>, DiagnosticError> {
    let byte_length = diagnostic_report[0].len();

    let mut bit_weights: Vec<i32> = vec![0; byte_length];

    for byte in diagnostic_report {
        if byte.len() > byte_length {
            return Err(DiagnosticError::ByteLengthError);
        }

        let bits = byte.chars();

        for (index, bit) in bits.enumerate() {
            match bit {
                '0' => {
                    bit_weights[index] -= 1;
                }
                '1' => {
                    bit_weights[index] += 1;
                }
                _ => {
                    return Err(DiagnosticError::ParseBinaryError);
                }
            };
        }
    }

    Ok(bit_weights)
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

/// Calculates the power consumption given a diagnostic report.
///
/// # Errors
/// If the diagnostic report contains non-binary bytes or bytes of differing lengths, or if the
/// gamma times the epsilon values cause an integer overflow when combined, this function will
/// return an Error.
pub fn calculate_power_consumption(diagnostic_report: &[String]) -> Result<i32, DiagnosticError> {
    if diagnostic_report.is_empty() {
        return Err(DiagnosticError::EmptyReportError);
    }

    let bit_weights = calculate_bit_weights(diagnostic_report)?;
    let gamma_string = convert_bit_weight_to_binary_string(&bit_weights);
    let epsilon_string = invert_binary_string(&gamma_string);
    let gamma_rate =
        i32::from_str_radix(&gamma_string, 2).map_err(|_| DiagnosticError::ImpossibleStateError)?;
    let epsilon_rate = i32::from_str_radix(&epsilon_string, 2)
        .map_err(|_| DiagnosticError::ImpossibleStateError)?;

    gamma_rate
        .checked_mul(epsilon_rate)
        .ok_or(DiagnosticError::OverflowError)
}
