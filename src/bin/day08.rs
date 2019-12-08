use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let width: usize = 25;
    let height: usize = 6;

    let layers: Vec<Vec<u8>> = input.as_bytes()
        .chunks(width * height)
        .map(|s| s.iter().map(|b| *b - b'0').collect())
        .collect();

    let min_layer = layers.iter()
        .min_by_key(|v| v.iter().filter(|b| **b == 0).count())
        .unwrap();

    let ones_count = min_layer.iter().filter(|b| **b == 1).count();
    let twos_count = min_layer.iter().filter(|b| **b == 2).count();

    println!("The number of 1s times the number of 2s: {}", ones_count * twos_count);

    for h in 0..height {
        for w in 0..width {
            for l in 0..layers.len() {
                let p = layers[l][h * width + w];
                if p != 2 {
                    print!("{}", if p == 0 { ' ' } else { '#' });
                    break;
                }
            }
        }
        println!("");
    }

    Ok(())
}
