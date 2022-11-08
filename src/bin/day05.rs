use std::io;
use std::io::prelude::*;

use intcode::Program;
use intcode::Number;

fn get_diagnostic_code(program: Vec<Number>, first_input: Number) -> Number {
    let mut p = Program::new(program);

    p.push_input(first_input);
    p.run_till_halted_or_blocked();
 
    p.last_output().unwrap()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input = input.split('\n').next().unwrap();
    let input: Vec<Number> = input.split(',').map(|s| s.parse::<Number>().unwrap()).collect();
    
    let program = input.clone();

    println!("Diagnostic code for system ID {}: {}", 1, get_diagnostic_code(program.clone(), 1));
    println!("Diagnostic code for system ID {}: {}", 5, get_diagnostic_code(program, 5));

    Ok(())
}
