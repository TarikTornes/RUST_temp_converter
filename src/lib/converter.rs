use crate::convert_utils;

pub fn temp_conv(input: &str) -> Option<(f64, char)> {


    let letter = match input.chars().rev().next() {

        Some(letter_pos) => letter_pos,
        None => return None,
    };


    let float_part = &input[..input.len() - 1];

    match float_part.parse::<f64>() {
        Ok(val) => {
            let result = match letter {
                'C' => convert_utils::celsius_to_fahrenheit(val),
                'F' => convert_utils::fahrenheit_to_celsius(val),
                _ => return None,
            };
            Some((result, letter))
        },
        Err(_) => None,
    }
}
