use std::io;
use std::io::prelude::*;

use intcode::Program;
use intcode::Number;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = input.split('\n').next().unwrap();
    let mut input: Vec<Number> = input.split(',').map(|s| s.parse::<Number>().unwrap()).collect();

    let mut program = Program::new(&input);
    program.run_till_halted_or_blocked();

    let mut num_blocks = 0;
    while program.has_output() {
        program.get_output().unwrap();
        program.get_output().unwrap();
        if program.get_output().unwrap() == 2 {
            num_blocks += 1;
        }
    }

    println!("Number of blocks in the game: {}", num_blocks);

    input[0] = 2;
    let mut program = Program::new(&input);
    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut score = 0;

    loop {
        program.run_till_halted_or_blocked();

        while program.has_output() {
            let x = program.get_output().unwrap();
            let y = program.get_output().unwrap();
            let tile = program.get_output().unwrap();

            if x == -1 && y == 0 {
                num_blocks -= 1;
                score = tile;
            }
            if tile == 3 {
                paddle_x = x;
            }
            if tile == 4 {
                ball_x = x;
            }
        }

        if num_blocks == 0 {
            break;
        }

        if paddle_x < ball_x {
            program.push_input(1);
        } else if paddle_x > ball_x {
            program.push_input(-1);
        } else {
            program.push_input(0);
        }
    }

    println!("Score when the game is over: {}", score);

    Ok(())
}
