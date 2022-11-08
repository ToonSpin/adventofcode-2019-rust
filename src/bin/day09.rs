use std::io;
use std::io::prelude::*;

use intcode::Program;
use intcode::Number;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input = input.split('\n').next().unwrap();
    let input: Vec<Number> = input.split(',').map(|s| s.parse::<Number>().unwrap()).collect();

    let mut program = Program::new(input.clone());
    program.push_input(1);
    program.run_till_halted_or_blocked();
    println!("The BOOST keycode: {}", program.get_output().unwrap());

    let mut program = Program::new(input);
    program.push_input(2);
    program.run_till_halted_or_blocked();
    println!("The location of the distress signal: {}", program.get_output().unwrap());

    Ok(())
}
