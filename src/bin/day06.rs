use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn orbit_str_to_tuple(s: &str) -> (&str, &str) {
    let mut split = s.split(')');
    (split.next().unwrap(), split.next().unwrap())
}

fn get_orbits_by_orbiter<'a>(orbiter: &'a str, orbits: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut s = orbiter;
    let mut v = Vec::new();
    while orbits.contains_key(&s) {
        v.push(*orbits.get(&s).unwrap());
        s = orbits.get(&s).unwrap();
    }
    v
}

fn get_orbit_count_by_orbiter(orbiter: &str, orbits: &HashMap<&str, &str>) -> usize {
    let mut count = 0;
    let mut s = orbiter;
    while orbits.contains_key(&s) {
        s = orbits.get(&s).unwrap();
        count += 1;
    }
    count
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split('\n').filter(|s| s.len() > 0).collect();

    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for (orbitee, orbiter) in input.iter().map(|s| orbit_str_to_tuple(s)) {
        orbits.insert(orbiter, orbitee);
    }

    let orbit_count: usize = orbits.keys().map(|s| get_orbit_count_by_orbiter(s, &orbits)).sum();

    println!("The total number of orbits in the solar system: {}", orbit_count);

    let orbits_you = get_orbits_by_orbiter("YOU", &orbits);
    let orbits_santa = get_orbits_by_orbiter("SAN", &orbits);

    let mut num_steps = orbits_you.len() + orbits_santa.len();
    let mut py = orbits_you.len() - 1;
    let mut ps = orbits_santa.len() - 1;

    while orbits_you[py] == orbits_santa[ps] {
        num_steps -= 2;
        py -= 1;
        ps -= 1;
    }

    println!("The number of steps that need to be taken is: {}", num_steps);

    Ok(())
}
