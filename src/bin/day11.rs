use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use intcode::Program;
use intcode::Number;

#[derive(Debug)]
enum Color {
    Black,
    White
}

impl Color {
    fn from(i: Number) -> Self {
        if i == 0 { Color::Black } else { Color::White }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

impl Direction {
    fn turn(&self, turn_dir: Number) -> Self {
        match self {
            North => if turn_dir == 0 { West } else { East },
            East => if turn_dir == 0 { North } else { South },
            South => if turn_dir == 0 { East } else { West },
            West => if turn_dir == 0 { South } else { North },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point (Number, Number);

impl Point {
    fn move_in_direction(&self, dir: Direction) -> Self {
        let (x, y) = (self.0, self.1);
        match dir {
            North => Point(x, y - 1),
            East => Point(x + 1, y),
            South => Point(x, y + 1),
            West => Point(x - 1, y),
        }
    }
}

fn paint(mut program: Program, initial_color: Color) -> HashMap<Point, Color> {
    let mut p = Point(0, 0);
    let mut dir = North;
    let mut hull: HashMap<Point, Color> = HashMap::new();
    hull.insert(p, initial_color);

    while !program.halted() {
        let color = hull.entry(p).or_insert(Color::Black);
        program.push_input(if let Color::Black = color { 0 } else { 1 });
        program.run_till_halted_or_blocked();
        if program.has_output() {
            *color = Color::from(program.get_output().unwrap());
            dir = dir.turn(program.get_output().unwrap());
            p = p.move_in_direction(dir);
        }
    }

    hull
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let input = input.split('\n').next().unwrap();
    let input: Vec<Number> = input.split(',').map(|s| s.parse::<Number>().unwrap()).collect();

    let hull = paint(Program::new(&input), Color::Black);
    println!("Number of cells that were painted: {}", hull.len());

    let hull = paint(Program::new(&input), Color::White);

    let mut field = Vec::new();
    for _y in 0..6 {
        field.push(vec![' '; 40])
    }
    for (p, col) in hull.iter() {
        let (x, y) = (p.0 - 1, p.1);
        if let Color::White = col {
            field[y as usize][x as usize] = '#';
        }
    }

    for v in field.iter() {
        for c in v.iter() {
            print!("{}", c);
        }
        println!("");
    }

    Ok(())
}
