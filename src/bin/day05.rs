use std::io;
use std::io::prelude::*;

use intcode::Program;

fn get_diagnostic_code(program: &Vec<i32>, first_input: i32) -> i32 {
    let mut p = Program::new(program);

    p.push_input(first_input);
    p.run_till_halted_or_blocked();
 
    p.last_output().unwrap()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input = input.split('\n').next().unwrap();
    let input: Vec<i32> = input.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    
    let program = input.clone();

    println!("Diagnostic code for system ID {}: {}", 1, get_diagnostic_code(&program, 1));
    println!("Diagnostic code for system ID {}: {}", 5, get_diagnostic_code(&program, 5));

    Ok(())
}
