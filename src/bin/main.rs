use std::io;
use temp_converter_lib::converter;

fn read_input() -> String {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input.trim().to_string()
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
        
        match converter::temp_conv(&input) {
            Some(tup) => print_res(tup),
            None => {
                println!("False input, retry again!");
                continue;
            }
        }
    }

}

