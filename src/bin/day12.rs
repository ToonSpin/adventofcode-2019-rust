use std::io;
use std::io::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::{map, map_res, opt, recognize},
    IResult,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated, tuple}
};

type Coordinate = i64;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(lcm(a, b), c)
}

#[derive(Clone, Copy, Debug)]
struct Moon {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
    vx: Coordinate,
    vy: Coordinate,
    vz: Coordinate,
}

impl Moon {
    fn new(x: Coordinate, y: Coordinate, z: Coordinate) -> Moon {
        Moon {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn get_potential_energy(&self) -> Coordinate {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn get_kinetic_energy(&self) -> Coordinate {
        self.vx.abs() + self.vy.abs() + self.vz.abs()
    }

    fn get_energy(&self) -> Coordinate {
        self.get_potential_energy() * self.get_kinetic_energy()
    }

    fn get_coordinates(&self) -> (Coordinate, Coordinate, Coordinate) {
        (self.x, self.y, self.z)
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn apply_attraction_from(&mut self, x: Coordinate, y: Coordinate, z: Coordinate) {
        if self.x < x {
            self.vx += 1;
        } else if self.x > x {
            self.vx -= 1;
        }
        if self.y < y {
            self.vy += 1;
        } else if self.y > y {
            self.vy -= 1;
        }
        if self.z < z {
            self.vz += 1;
        } else if self.z > z {
            self.vz -= 1;
        }
    }
}

#[derive(Debug)]
struct LunarSystem {
    moons: Vec<Moon>,
    x_cycled: Option<u64>,
    y_cycled: Option<u64>,
    z_cycled: Option<u64>,
}

impl LunarSystem {
    fn new(moons: &Vec<Moon>) -> LunarSystem {
        LunarSystem {
            moons: moons.clone(),
            x_cycled: None,
            y_cycled: None,
            z_cycled: None,
        }
    }

    fn get_energy(&self) -> Coordinate {
        self.moons.iter().map(|m| m.get_energy()).sum()
    }

    fn step(&mut self) {
        for i in 0..self.moons.len() {
            for j in i+1..self.moons.len() {
                let (x, y, z) = self.moons[j].get_coordinates();
                self.moons[i].apply_attraction_from(x, y, z);
                let (x, y, z) = self.moons[i].get_coordinates();
                self.moons[j].apply_attraction_from(x, y, z);
            }
        }
        for moon in self.moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    fn get_cycles(&mut self) -> (u64, u64, u64) {
        let mut i = 0;
        loop {
            let mut done = true; 
            self.step();
            i += 1;
            if let None = self.x_cycled {
                if self.zero_x_velocity() {
                    self.x_cycled = Some(i);
                } else {
                    done = false;
                }
            }
            if let None = self.y_cycled {
                if self.zero_y_velocity() {
                    self.y_cycled = Some(i);
                } else {
                    done = false;
                }
            }
            if let None = self.z_cycled {
                if self.zero_z_velocity() {
                    self.z_cycled = Some(i);
                } else {
                    done = false;
                }
            }
            if done {
                break;
            }
        }
        return (
            self.x_cycled.unwrap(),
            self.y_cycled.unwrap(),
            self.z_cycled.unwrap()
        )
    }

    fn zero_x_velocity(&mut self) -> bool {
        for m in self.moons.iter() {
            if m.vx != 0 {
                return false;
            }
        }
        true
    }

    fn zero_y_velocity(&mut self) -> bool {
        for m in self.moons.iter() {
            if m.vy != 0 {
                return false;
            }
        }
        true
    }

    fn zero_z_velocity(&mut self) -> bool {
        for m in self.moons.iter() {
            if m.vz != 0 {
                return false;
            }
        }
        true
    }
}

fn parse_coordinate_triple(input: &str) -> IResult <&str, (Coordinate, Coordinate, Coordinate)> {
    let prefixed_coordinate = preceded(pair(anychar, char('=')), parse_coordinate);
    tuple((
        terminated(preceded(pair(anychar, char('=')), parse_coordinate), tag(", ")),
        terminated(preceded(pair(anychar, char('=')), parse_coordinate), tag(", ")),
        prefixed_coordinate,
    ))(input)
}

fn parse_coordinate(input: &str) -> IResult <&str, Coordinate> {
    let parser = pair(opt(char('-')), digit1);
    map_res(recognize(parser), |s: &str| s.parse::<Coordinate>())(input)
}

fn parse_moon(input: &str) -> IResult <&str, Moon> {
    let parser = delimited(char('<'), parse_coordinate_triple, char('>'));
    map(parser, |t: (Coordinate, Coordinate, Coordinate)| Moon::new(t.0, t.1, t.2))(input)
}

fn parse_moons(input: &str) -> IResult <&str, Vec<Moon>> {
    separated_list1(char('\n'), parse_moon)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let (_rest, input) = parse_moons(input).unwrap();
    
    let mut system = LunarSystem::new(&input);
    for _i in 0..1000 {
        system.step();
    }
    println!("Total energy in the system after 100 steps: {}", system.get_energy());

    let mut system = LunarSystem::new(&input);
    let (cx, cy, cz) = system.get_cycles();
    println!("The number of steps before the first cycle: {}", lcm3(cx, cy, cz) * 2);

    Ok(())
}
