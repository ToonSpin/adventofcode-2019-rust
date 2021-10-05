use std::io;
use std::io::prelude::*;

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res, value, verify},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn horizontal(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => false,
            Direction::Left | Direction::Right => true,
        }
    }
    fn vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            Direction::Left | Direction::Right => false,
        }
    }
}

struct Segment {
    dir: Direction,
    dist: i32,
    x: i32,
    y: i32,
    steps: i32,
}

impl Segment {
    fn delta(&self) -> (i32, i32) {
        match self.dir {
            Direction::Up => (0, self.dist),
            Direction::Down => (0, -self.dist),
            Direction::Left => (-self.dist, 0),
            Direction::Right => (self.dist, 0),
        }
    }

    fn contains(&self, point: &(i32, i32)) -> bool {
        let (dx, dy) = self.delta();
        let (p, q) = point;
        let minx;
        let maxx;
        let miny;
        let maxy;

        if dx < 0 {
            minx = self.x + dx;
            maxx = self.x;
        } else {
            minx = self.x;
            maxx = self.x + dx;
        }
        if dy < 0 {
            miny = self.y + dy;
            maxy = self.y;
        } else {
            miny = self.y;
            maxy = self.y + dy;
        }

        *p >= minx && *p <= maxx && *q >= miny && *q <= maxy
    }
}

fn parse_direction_up(input: &str) -> IResult<&str, Direction> {
    value(Direction::Up, char('U'))(input)
}

fn parse_direction_down(input: &str) -> IResult<&str, Direction> {
    value(Direction::Down, char('D'))(input)
}

fn parse_direction_left(input: &str) -> IResult<&str, Direction> {
    value(Direction::Left, char('L'))(input)
}

fn parse_direction_right(input: &str) -> IResult<&str, Direction> {
    value(Direction::Right, char('R'))(input)
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        parse_direction_up,
        parse_direction_down,
        parse_direction_left,
        parse_direction_right,
    ))(input)
}

fn parse_distance(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn parse_segment(input: &str) -> IResult<&str, Segment> {
    let parser = tuple((parse_direction, parse_distance));
    map(parser, |(dir, dist)| Segment {
        dir,
        dist,
        x: 0,
        y: 0,
        steps: 0,
    })(input)
}

fn parse_wire(input: &str) -> IResult<&str, Vec<Segment>> {
    let wire_parser = separated_list1(char(','), parse_segment);
    verify(wire_parser, |v: &Vec<Segment>| v.len() > 1)(input)
}

fn parse_wires(input: &str) -> IResult<&str, Vec<Vec<Segment>>> {
    separated_list1(char('\n'), parse_wire)(input)
}

fn get_intersections(v1: &Vec<Segment>, v2: &Vec<Segment>) -> Vec<(i32, i32, i32)> {
    let mut intersections = Vec::new();
    for a in v1.iter() {
        for b in v2.iter() {
            if a.dir.horizontal() && b.dir.vertical() {
                let intersection = (b.x, a.y);
                if a.contains(&intersection) && b.contains(&intersection) {
                    let steps_a = a.steps + (b.x - a.x).abs();
                    let steps_b = b.steps + (b.y - a.y).abs();
                    intersections.push((intersection.0, intersection.1, steps_a + steps_b));
                }
            }
            if a.dir.vertical() && b.dir.horizontal() {
                let intersection = (a.x, b.y);
                if a.contains(&intersection) && b.contains(&intersection) {
                    let steps_a = a.steps + (b.y - a.y).abs();
                    let steps_b = b.steps + (b.x - a.x).abs();
                    intersections.push((intersection.0, intersection.1, steps_a + steps_b));
                }
            }
        }
    }
    intersections
}

fn fill_coordinates_and_steps(v: &mut Vec<Segment>) {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;

    for segment in v.iter_mut() {
        segment.x = x;
        segment.y = y;
        segment.steps = steps;

        let (dx, dy) = segment.delta();

        x += dx;
        y += dy;
        steps += segment.dist;
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let (_rest, mut input) = parse_wires(input).unwrap();

    fill_coordinates_and_steps(&mut input[0]);
    fill_coordinates_and_steps(&mut input[1]);

    let min_distance: i32 = get_intersections(&input[0], &input[1])
        .iter()
        .map(|(x, y, _s)| x.abs() + y.abs())
        .min()
        .unwrap();
    let min_steps: i32 = get_intersections(&input[0], &input[1])
        .iter()
        .map(|(_x, _y, s)| *s)
        .min()
        .unwrap();

    println!("The distance of the closest intersection is: {}", min_distance);
    println!("The minimum combined number of steps: {}", min_steps);

    Ok(())
}
