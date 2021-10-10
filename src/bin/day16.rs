use std::io;
use std::io::prelude::*;

fn double_vec(v: Vec<i32>) -> Vec<i32> {
    [&v[..], &v[..]].concat()
}

fn quintuple_vec(v: Vec<i32>) -> Vec<i32> {
    [&v[..], &v[..], &v[..], &v[..], &v[..]].concat()
}

fn fft_phase(input: &Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::with_capacity(input.len());

    for digit in 0..input.len() {
        let mut sum = 0;
        let mut index = digit;

        while index < input.len() {
            let mut multiplier = ((index + 1) / (digit + 1)) as i32;
            if multiplier % 4 == 3 {
                multiplier = -1;
            } else if multiplier % 4 == 1 {
                multiplier = 1;
            } else {
                index += digit + 1;
                continue;
            }
            sum += multiplier * input[index % input.len()];
            index += 1;
        }

        output.push((sum % 10).abs());
    }
    output
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input: Vec<i32> = input.as_bytes().iter().filter(|b| **b >= b'0').map(|b| (*b - b'0') as i32).collect();

    let mut temp = input.clone();
    for _i in 0..100 {
        temp = fft_phase(&temp);
    }
    print!("The output after 100 phases of FFT: ");
    for i in 0..8 {
        print!("{}", temp[i]);
    }
    println!("");

    let skip_digits = input[..7].iter().fold(0, |a, i| a * 10 + i) as usize;
    if skip_digits * 2 < input.len() {
        panic!("Can't compute part 2 for this input");
    }
    let temp = input.clone();
    let temp = quintuple_vec(temp);
    let temp = quintuple_vec(temp);
    let temp = quintuple_vec(temp);
    let temp = quintuple_vec(temp);
    let temp = double_vec(temp);
    let temp = double_vec(temp);
    let temp = double_vec(temp);
    let mut temp = double_vec(temp)[skip_digits..].to_vec();

    for _i in 0..100 {
        let mut digit = temp.len() - 1;
        let mut sum = 0;
        loop {
            sum += temp[digit];
            temp[digit] = (sum % 10).abs();
            if digit == 0 {
                break;
            }
            digit -= 1;
        }
    }
    print!("The message embedded in the final output list: ");
    for i in 0..8 {
        print!("{}", temp[i]);
    }
    println!("");
    Ok(())
}
