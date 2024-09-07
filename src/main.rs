use std::io;

const FACTOR: f64 = 1.8;


fn read_input() -> String {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input.trim().to_string()
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * FACTOR + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / FACTOR
}

fn temp_conv(input: &str) -> Option<(f64, char)> {


    let letter = match input.chars().rev().next() {

        Some(letter_pos) => letter_pos,
        None => return None,
    };


    let float_part = &input[..input.len() - 1];

    match float_part.parse::<f64>() {
        Ok(val) => {
            let result = match letter {
                'C' => celsius_to_fahrenheit(val),
                'F' => fahrenheit_to_celsius(val),
                _ => return None,
            };
            Some((result, letter))
        },
        Err(_) => None,
    }
}

fn print_res(tup: (f64, char)) {

    let (float, letter) = tup;

    match letter {
        'C' => println!("It's {:.2}°F", float),
        'F' => println!("It's {:.2}°C", float),
        _ => println!("False input, retry again!"),
    }
}
    


fn main() {


    println!("Celsius -> Fahrenheit: Append the character \"C\" to the number");
    println!("Fahrenheit -> Celsius: Append the character \"F\" to the number");
    
    loop {

        let input = read_input();
        
        match temp_conv(&input) {
            Some(tup) => print_res(tup),
            None => {
                println!("False input, retry again!");
                continue;
            }
        }
    }

}

