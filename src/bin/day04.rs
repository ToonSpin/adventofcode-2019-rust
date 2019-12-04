use std::io;
use std::io::prelude::*;

fn digits(mut n: u32) -> Vec<u32> {
    let mut v = Vec::with_capacity(6);
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v
}

fn valid_password(n: u32) -> bool {
    let mut prev = None;
    let mut found_equal = false;

    for digit in digits(n) {
        if let Some(prev_digit) = prev {
            if prev_digit < digit {
                return false;
            }

            if prev_digit == digit {
                found_equal = true;
            }
        }
        prev = Some(digit);
    }

    found_equal
}

fn password_contains_pair(n: u32) -> bool {
    let mut prev = None;
    let mut equal_count = 1;

    for digit in digits(n) {
        if let Some(prev_digit) = prev {
            if prev_digit == digit {
                equal_count += 1;
            } else {
                if equal_count == 2 {
                    return true;
                }
                equal_count = 1;
            }
        }
        prev = Some(digit);
    }
    equal_count == 2
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split('-').collect();

    let a: u32 = input[0].parse().unwrap();
    let b: u32 = input[1].parse().unwrap();

    let count_part1 = (a..=b).filter(|i| valid_password(*i)).count();
    let count_part2 = (a..=b).filter(|i| valid_password(*i)).filter(|i| password_contains_pair(*i)).count();

    println!("Number of passwords that meet the criteria: {}", count_part1);
    println!("Number of passwords that meet the stricter criteria: {}", count_part2);

    Ok(())
}
