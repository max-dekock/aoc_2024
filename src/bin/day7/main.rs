use nom::{
    IResult,
    bytes::complete::{tag},
    character::complete::{space1, u64},
    multi::{separated_list1},
    sequence::{terminated, separated_pair},
};
use std::collections::HashSet;

#[derive(Debug)]
struct Equation {
    output: u64,
    inputs: Vec<u64>
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let output = terminated(u64, tag(":"));
    let inputs = separated_list1(space1, u64);
    separated_pair(output, space1, inputs)(input)
        .map(|(rest, (output, inputs))| (rest, Equation { output, inputs }))
}

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| parse_equation(line).map(|(_, eq)| eq))
        .collect::<Result<Vec<Equation>, _>>()
        .unwrap()
}

fn is_possible(eq: &Equation, ops: &[&dyn Fn(u64, u64) -> u64]) -> bool {
    eq.inputs.iter()
        .fold(HashSet::new(), |possibilities, &input| {
            if possibilities.len() == 0 {
                let mut new_possibilities = HashSet::new();
                new_possibilities.insert(input);
                return new_possibilities;
            } else {
                let mut new_possibilities = HashSet::new();
                for p in possibilities {
                    for op in ops {
                        new_possibilities.insert(op(p, input));
                    }
                }
                return new_possibilities;
            }
        })
        .contains(&eq.output)
}

fn part_1(input: &str) -> u64 {
    let equations = parse_input(input);
    let mut sum = 0;
    for eq in equations {
        if is_possible(&eq, &[
            &|x, y| x + y,
            &|x, y| x * y
        ]) {
            sum += eq.output;
        }
    }
    sum
}

fn concat_numbers(x: u64, y: u64) -> u64 {
    let y_digit_count = y.ilog10() + 1;
    x * 10_u64.pow(y_digit_count) + y
}

fn part_2(input: &str) -> u64 {
    let equations = parse_input(input);
    let mut sum = 0;
    for eq in equations {
        if is_possible(&eq, &[
            &|x, y| x + y,
            &|x, y| x * y,
            &concat_numbers
        ]) {
            sum += eq.output;
        }
    }
    sum
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
