use nom::{
    IResult,
    Parser,
    error::Error,
    bytes::complete::tag,
    character::complete::{i64, space1, multispace1, line_ending},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, tuple}
};

use std::collections::HashMap;

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

fn parse_machine_parameter<'a>(label: &'a str, op: &'a str) -> impl Parser<&'a str, (i64, i64), Error<&'a str>>
{
    preceded(
        tuple((tag(label), tag(":"), space1)),
        separated_pair(
            preceded(pair(tag("X"), tag(op)), i64),
            pair(tag(","), space1),
            preceded(pair(tag("Y"), tag(op)), i64),
        )
    )
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, button_a) = parse_machine_parameter("Button A", "+").parse(input)?;
    let (input, _) = line_ending(input)?;
    let (input, button_b) = parse_machine_parameter("Button B", "+").parse(input)?;
    let (input, _) = line_ending(input)?;
    let (input, prize) = parse_machine_parameter("Prize", "=").parse(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Machine { button_a, button_b, prize }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(multispace1, parse_machine)(input)
}

const A_TOKENS: u64 = 3;
const B_TOKENS: u64 = 1;

//failed first attempt with dynamic programming
fn cost_to_reach(coord: (i64, i64), machine: &Machine, cache: &mut HashMap<(i64, i64), u64>) -> Option<u64> {
    println!("entering {:?}", coord);
    if coord.0 == 0 && coord.1 == 0 {
        return Some(0);
    }
    if coord.0 < 0 || coord.1 < 0 {
        return None;
    }
    if let Some(cost) = cache.get(&coord) {
        return Some(*cost);
    }
    let cost_from_a = cost_to_reach((coord.0 - machine.button_a.0, coord.1 - machine.button_a.1), machine, cache);
    let cost_from_b = cost_to_reach((coord.0 - machine.button_b.0, coord.1 - machine.button_b.1), machine, cache);
    let best_cost = match (cost_from_a, cost_from_b) {
        (Some(a), Some(b)) => Some(std::cmp::min(a + A_TOKENS, b + B_TOKENS)),
        (Some(a), None) => Some(a + A_TOKENS),
        (None, Some(b)) => Some(b + B_TOKENS),
        (None, None) => None
    };
    if let Some(cost) = best_cost {
        cache.insert(coord, cost);
    }
    println!("exiting {:?}", coord);
    best_cost
}

fn tokens_required(machine: &Machine) -> Option<u64> {
    let &Machine { button_a, button_b, prize } = machine;
    for num_a in 0.. {
        let ax = button_a.0 * num_a;
        let ay = button_a.1 * num_a;
        if ax > prize.0 || ay > prize.1 {
            return None;
        }
        let bx = prize.0 - ax;
        let by = prize.1 - ay;
        let num_b = bx / button_b.0;
        if button_b.0 * num_b == bx && button_b.1 * num_b == by {
            return Some(num_a as u64 * A_TOKENS + num_b as u64 * B_TOKENS);
        }
    }
    None
}
        
fn part_1(input: &str) -> u64 {    
    let (_, machines) = parse_input(input).unwrap();
    let mut total_cost = 0;
    for machine in &machines {
        total_cost += tokens_required(machine).unwrap_or(0);
    }
    total_cost
}

fn extended_euclidean(x: i64, y: i64) -> (i64, i64, i64) {
    let (mut r_prev, mut r) = (x, y);
    let (mut s_prev, mut s) = (1, 0);
    let (mut t_prev, mut t) = (0, 1);
    while r > 0 {
        let q = r_prev / r;
        (r_prev, r) = (r, r_prev - q * r);
        (s_prev, s) = (s, s_prev - q * s);
        (t_prev, t) = (t, t_prev - q * t);
    }
    (r_prev, s_prev, t_prev)
}

fn div_ceil(a: i64, b: i64) -> i64 {
    let mut q = a / b;
    let r = a % b;
    if r != 0 {
        q += 1;
    }
    q
}

// improved version using Bezout's identity
fn tokens_required_improved(machine: &Machine) -> Option<i64> {
    let Machine { button_a, button_b, prize } = machine;
    let (gcd, bezout_a, bezout_b) = extended_euclidean(button_a.0, button_b.0);
    if prize.0 % gcd != 0 {
        return None;
    }
    let multiple = prize.0 / gcd;
    let (mut num_a, mut num_b) = (bezout_a * multiple, bezout_b * multiple);
    println!("{}, {}", num_a, num_b);
    let (delta_a, delta_b) = (button_a.0 / gcd, button_b.0 / gcd);
    println!("{}, {}", delta_a, delta_b);
    let k = div_ceil(-num_a, delta_a);
    println!("k = {}", k);
    num_a += k * delta_a;
    num_b -= k * delta_b;
    if num_a < 0 || num_b < 0 {
        return None;
    }
    println!("a = {}, b = {}", num_a, num_b);

    assert!(num_a * button_a.0 + num_b * button_b.0 == prize.0, "ended on wrong x coord");
    assert!(num_a * button_a.1 + num_b * button_b.1 == prize.1, "ended on wrong y coord");
    Some(num_a * A_TOKENS as i64 + num_b * B_TOKENS as i64)
}

fn part_2(input: &str) -> i64 {
    let (_, mut machines) = parse_input(input).unwrap();
    machines.iter_mut().for_each(|machine| {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
    });
    let mut total_cost = 0;
    for machine in &machines {
        total_cost += tokens_required_improved(machine).unwrap_or(0);
    }
    total_cost
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
