use std::error::Error;
use std::io;

pub fn handle_choice() -> Result<u32, Box<dyn Error>> {
    let mut input_choice: String = String::new();
    io::stdin().read_line(&mut input_choice)?;
    let input_choice = input_choice.trim().parse::<u32>()?;
    Ok(input_choice)
}

pub fn handle_input() -> Result<String, Box<dyn Error>> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    Ok(input)
}
