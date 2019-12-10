use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

use intcode::Program;
use intcode::Number;

fn get_result_part1(settings: Vec<Number>, program: &Vec<Number>) -> Number {
    let mut output = 0;

    for setting in settings.iter() {
        let mut p = Program::new(&program);
        p.push_input(*setting);
        p.push_input(output);
        p.run_till_halted_or_blocked();
        output = p.get_output().unwrap();
    }

    output
}

fn get_result_part2(settings: Vec<Number>, program: &Vec<Number>) -> Number {
    let mut programs: Vec<Program> = Vec::new();
    let num_programs = settings.len();

    for setting in settings.iter() {
        let mut p = Program::new(&program);
        p.push_input(*setting);
        programs.push(p);
    }

    programs[0].push_input(0);

    while !programs[num_programs - 1].halted() {
        for i in 0..num_programs {
            programs[i].run_till_halted_or_blocked();
            let j = (i + 1) % num_programs;
            while programs[i].has_output() {
                let output = programs[i].get_output().unwrap();
                programs[j].push_input(output);
            }
        }
    }
    programs[num_programs - 1].last_output().unwrap()
}

#[test]
fn test_part1() {
    let v = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    assert_eq!(get_result_part1(vec![4,3,2,1,0], &v), 43210);

    let v = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
    assert_eq!(get_result_part1(vec![0,1,2,3,4], &v), 54321);

    let v = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    assert_eq!(get_result_part1(vec![1,0,4,3,2], &v), 65210);
}

#[test]
fn test_part2() {
    let v = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
    assert_eq!(get_result_part2(vec![9,8,7,6,5], &v), 139629729);

    let v = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
    assert_eq!(get_result_part2(vec![9,7,8,5,6], &v), 18216);
}

fn valid_input(input: &Vec<Number>) -> bool {
    let mut found: HashSet<Number> = HashSet::new();
    for i in input.iter() {
        if found.insert(*i) == false {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input = input.split('\n').next().unwrap();
    let input: Vec<Number> = input.split(',').map(|s| s.parse::<Number>().unwrap()).collect();
    
    let program = input.clone();

    let mut max_output = 0;
    for a in 0..=4 {
        for b in 0..=4 {
            for c in 0..=4 {
                for d in 0..=4 {
                    for e in 0..=4 {
                        let program_input = vec![a, b, c, d, e];
                        if valid_input(&program_input) {
                            let result = get_result_part1(program_input, &program);
                            if result > max_output {
                                max_output = result;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("The highest possible signal is: {}", max_output);

    let mut max_output = 0;
    for a in 5..=9 {
        for b in 5..=9 {
            for c in 5..=9 {
                for d in 5..=9 {
                    for e in 5..=9 {
                        let program_input = vec![a, b, c, d, e];
                        if valid_input(&program_input) {
                            let result = get_result_part2(program_input, &program);
                            if result > max_output {
                                max_output = result;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("The highest possible signal with a feeback loop: {}", max_output);

    Ok(())
}
