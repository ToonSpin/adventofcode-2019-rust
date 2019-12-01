use std::io;
use std::io::prelude::*;

fn get_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_fuel_part_2(mass: i32) -> i32 {
    let mut total = 0;
    let mut subtotal = mass;
    loop {
        subtotal = get_fuel(subtotal);
        if subtotal <= 0 {
            break;
        }
        total += subtotal;
    }
    total
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input: Vec<i32> = input
        .split("\n")
        .filter(|l| *l != "")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let sum: i32 = input.iter().map(|i| get_fuel(*i)).sum();
    println!("The fuel required: {}", sum);

    let sum: i32 = input.iter().map(|i| get_fuel_part_2(*i)).sum();
    println!("The fuel required, taking into account fuel mass: {}", sum);

    Ok(())
}
