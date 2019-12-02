use std::io;
use std::io::prelude::*;

fn output(input_v: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut input = input_v.clone();
    input[1] = noun;
    input[2] = verb;

    let mut pos: usize = 0;
    loop {
        let op = input[pos];
        let a = input[pos + 1];
        let b = input[pos + 2];
        let c = input[pos + 3];

        match op {
            1 => { input[c] = input[a] + input[b]; }
            2 => { input[c] = input[a] * input[b]; }
            99 => { break; }
            _ => { unreachable!(); }
        }
        pos += 4;
    }
    input[0]
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input: Vec<usize> = input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    println!("{:?}", output(&input, 2, 12));

    for noun in 0..100 {
        for verb in 0..100 {
            if output(&input, noun, verb) == 19690720 {
                println!("{:?}", 100 * noun + verb);
                break;
            }
        }
    }

    Ok(())
}
