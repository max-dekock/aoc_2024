enum BlinkResult {
    Single(u64),
    Double(u64, u64)
}

use BlinkResult::*;

fn blink_stone(stone: u64) -> BlinkResult {
    if stone == 0 {
        return Single(1);
    }
    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
        let divisor = 10u64.pow(num_digits / 2);
        let left = stone / divisor;
        let right = stone % divisor;
        return Double(left, right);
    } else {
        return Single(stone.checked_mul(2024).unwrap());
    }
}

fn blink_sequence(stones: &[u64]) -> Vec<u64> {
    let mut result = vec![];
    for &stone in stones {
        match blink_stone(stone) {
            Single(x) => result.push(x),
            Double(x, y) => {
                result.push(x);
                result.push(y);
            }
        };
    }
    result
}

fn parse(input: &str) -> Vec<u64> {
    return input.split_whitespace()
        .map(|token| token.parse::<u64>().unwrap())
        .collect();
}

fn part_1(input: &str) -> usize {
    let mut stones = parse(input);

    for _ in 0..25 {
        stones = blink_sequence(&stones);
    }
    return stones.len();
}

use std::collections::HashMap;

fn num_children(stone: u64, gens_remaining: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if gens_remaining == 0 {
        return 1;
    }
    if let Some(&n) = cache.get(&(stone, gens_remaining)) {
        return n;
    }
    let n = match blink_stone(stone) {
        Single(x) => num_children(x, gens_remaining - 1, cache),
        Double(x, y) => num_children(x, gens_remaining - 1, cache) + num_children(y, gens_remaining - 1, cache)
    };
    cache.insert((stone, gens_remaining), n);
    return n;
}

fn part_2(input: &str) -> u64 {
    let stones = parse(input);
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    let mut total = 0;
    for stone in stones {
        total += num_children(stone, 75, &mut cache);
    }
    total
}


fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
