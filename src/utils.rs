use std::io;

pub fn handle_choice() -> Result<u32, &'static str> {
    let mut input_choice: String = String::new();
    match io::stdin().read_line(&mut input_choice) {
        Ok(_) => match input_choice.trim().parse::<u32>() {
            Ok(res) => Ok(res),
            Err(_) => Err("Error Parsing"),
        },
        Err(_) => Err("Error reading input"),
    }
    // let input_choice = input_choice.trim().parse::<u32>().expect("Cannot parse");

    // Ok(input_choice)
}

pub fn handle_input() -> Result<String, &'static str> {
    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
            Ok(input)
        }
        Err(_) => Err("Error reading input"),
    }
}
