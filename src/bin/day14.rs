use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res},
    IResult,
    multi::separated_list1,
    sequence::separated_pair
};

#[derive(Clone, Debug)]
struct ResourceSpecifier {
    num: u64,
    resource: String,
}

impl ResourceSpecifier {
    fn new(num: u64, resource: String) -> Self {
        ResourceSpecifier {
            num,
            resource
        }
    }
}

#[derive(Clone, Debug)]
struct ResourceRule {
    product: ResourceSpecifier,
    substrates: Vec<ResourceSpecifier>,
}

impl ResourceRule {
    fn new(product: ResourceSpecifier, substrates: Vec<ResourceSpecifier>) -> Self {
        ResourceRule {
            product,
            substrates
        }
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

fn parse_resource(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| String::from(s))(input)
}

fn parse_resource_specifier(input: &str) -> IResult<&str, ResourceSpecifier> {
    let parser = separated_pair(parse_u64, char(' '), parse_resource);
    map(parser, |(num, resource)| ResourceSpecifier::new(num, resource))(input)
}

fn parse_resource_rule_tuple(input: &str) -> IResult<&str, (Vec<ResourceSpecifier>, ResourceSpecifier)> {
    separated_pair(
        separated_list1(tag(", "), parse_resource_specifier),
        tag(" => "),
        parse_resource_specifier
    )(input)
}

fn parse_resource_rule(input: &str) -> IResult<&str, ResourceRule> {
    map(parse_resource_rule_tuple, |(substrates, product)| ResourceRule::new(product, substrates))(input)
}

fn parse_resource_rules_vec(input: &str) -> IResult<&str, Vec<ResourceRule>> {
    separated_list1(char('\n'), parse_resource_rule)(input)
}

fn parse_resource_rules_hashmap(input: &str) -> IResult<&str, HashMap<String, ResourceRule>> {
    let mut map: HashMap<String, ResourceRule> = HashMap::new();
    let (rest, v) = parse_resource_rules_vec(input)?;
    for r in v.iter() {
        let resource = &r.product.resource;
        let rule = r.clone();
        map.insert(resource.to_string(), rule);
    }
    Ok((rest, map))
}

fn get_ore_requirements(input: &HashMap<String, ResourceRule>, fuel_required: u64) -> u64 {
    let mut requirements: HashMap<String, u64> = HashMap::new();
    let mut stock: HashMap<String, u64> = HashMap::new();

    requirements.insert(String::from("FUEL"), fuel_required);
    let ore_string = String::from("ORE");

    let mut done;
    loop {
        done = true;
        let mut resource_to_produce = String::from("");
        let mut number_required = 0;

        for (resource, num) in requirements.iter() {
            if *resource == ore_string {
                continue;
            }
            done = false;
            number_required = *num;
            resource_to_produce = resource.clone();

            let in_stock = stock.entry(resource_to_produce.clone()).or_insert(0);

            if *in_stock > number_required {
                *in_stock -= number_required;
                number_required = 0;
            } else {
                number_required -= *in_stock;
                *in_stock = 0;
            }
            break;
        }

        if done {
            break;
        }

        let rule = input.get(&resource_to_produce).unwrap();
        let yield_per_iteration = rule.product.num;
        let mut num_iterations = number_required / yield_per_iteration;

        if number_required % yield_per_iteration > 0 {
            num_iterations += 1;
            let in_stock = stock.entry(resource_to_produce.clone()).or_insert(0);
            let surplus = num_iterations * yield_per_iteration - number_required;
            *in_stock += surplus;
        }
        requirements.remove(&resource_to_produce);
        for substrate in rule.substrates.iter() {
            let current_num = requirements.entry(substrate.resource.clone()).or_insert(0);
            *current_num += num_iterations * substrate.num;
        }
    }
    *requirements.get(&String::from("ORE")).unwrap()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let (_rest, input) = parse_resource_rules_hashmap(input).unwrap();

    let ore_required = get_ore_requirements(&input, 1);
    println!("Ore required for one unit of fuel: {}", ore_required);

    let mut min_fuel = 1;
    let mut max_fuel = 1;
    let ore_in_stock = 1_000_000_000_000;

    while get_ore_requirements(&input, max_fuel) < ore_in_stock {
        max_fuel *= 2;
    }

    while max_fuel - min_fuel > 1 {
        let pivot = (max_fuel - min_fuel) / 2 + min_fuel;

        let ore_required = get_ore_requirements(&input, pivot);

        if ore_required > ore_in_stock {
            max_fuel = pivot;
        } else if ore_required < ore_in_stock {
            min_fuel = pivot;
        } else {
            min_fuel = pivot;
            break;
        }
    }

    println!("Fuel that can be produced with one trillion ore: {}", min_fuel);
    Ok(())
}
