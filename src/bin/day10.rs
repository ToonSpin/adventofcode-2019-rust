use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn asteroid_in_sight(input: &Vec<Vec<u8>>, xc: i32, yc: i32, dir: (i32, i32)) -> Option<(i32, i32)> {
    let mut x = xc;
    let mut y = yc;

    let num_x = input[0].len() as i32;
    let num_y = input.len() as i32;

    let (p, q) = dir;

    loop {
        x += p;
        y += q;

        if x < 0 || y < 0 || x >= num_x || y >= num_y {
            break;
        }

        if input[y as usize][x as usize] == 1 {
            return Some((x, y));
        }
    }
    None
}

fn get_num_in_sight(input: &Vec<Vec<u8>>, xc: i32, yc: i32) -> u32 {
    let mut count = 0;
    let mut found: HashSet<(i32, i32)> = HashSet::new();

    let num_x = input[0].len() as i32;
    let num_y = input.len() as i32;

    for q in 0..num_y {
        for p in 0..num_x {
            let (p, q) = simplify_fraction(p - xc, q - yc);
            if p == 0 && q == 0 {
                continue;
            }
            if found.insert((p, q)) {
                if let Some(_) = asteroid_in_sight(&input, xc, yc, (p, q)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn get_unique_fractions(num: i32) -> HashSet<(i32, i32)> {
    let mut s = HashSet::new();
    for i in -num..=num {
        for j in -num..=num {
            if i != 0 || j != 0 {
                s.insert(simplify_fraction(i, j));
            }
        }
    }
    s
}

fn simplify_fraction(mut a: i32, mut b: i32) -> (i32, i32) {
    let g = gcd(a.abs(), b.abs());
    if g > 0 {
        a = a / g;
        b = b / g;
    }
    (a, b)
}

/// This works. I do not know how.
fn vector_to_angle(v: (i32, i32)) -> f64 {
    let (a, b) = v;

    let x = a as f64;
    let y = b as f64;

    let mut angle = y.atan2(x);
    angle += std::f64::consts::FRAC_PI_2;
    if angle < 0.0 {
        angle += 2f64 * std::f64::consts::PI;
    }
    angle
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input: Vec<&str> = input.split('\n').filter(|l| *l != "").collect();
    let input: Vec<Vec<u8>> = input.iter().map(|s| s.as_bytes().to_vec()).collect();
    let input: Vec<Vec<u8>> = input.iter().map(|v| v.iter().map(|b| if *b == b'.' { 0 } else { 1 }).collect()).collect();

    let num_x = input[0].len() as i32;
    let num_y = input.len() as i32;
    let max_dim = if num_x > num_y { num_x } else { num_y };

    let mut max_count = 0;
    let mut location = (0, 0);
    for q in 0..num_y {
        for p in 0..num_x {
            if input[q as usize][p as usize] == 1 {
                let count = get_num_in_sight(&input, p, q);
                if count > max_count {
                    max_count = count;
                    location = (p, q);
                }
            }
        }
    }
    println!("Number of asteroids in sight from best position: {}", max_count);

    let mut fractions: Vec<(i32, i32)> = get_unique_fractions(max_dim).iter().cloned().collect();
    fractions.sort_by_key(|t| (vector_to_angle(*t) * 10000f64) as i64);

    let mut field = input.clone();
    let mut index = 0;
    let mut asteroid_count = 0;
    let mut xa = 0;
    let mut ya = 0;
    let (xc, yc) = location;

    while asteroid_count < 200 {
        let dir = fractions[index];

        if let Some((p, q)) = asteroid_in_sight(&field, xc, yc, dir) {
            xa = p;
            ya = q;
            field[q as usize][p as usize] = 0;
            asteroid_count += 1;
        }

        index += 1;
        index %= fractions.len();
    }
    println!("The position of the 200th asteroid: {}", xa * 100 + ya);
    Ok(())
}
