use std::collections::{HashSet, HashMap};
use std::borrow::Borrow;

fn antinodes<A: Borrow<(i32, i32)>>(a1: A, a2: A) -> [(i32, i32); 2] {
    let a1 = a1.borrow();
    let a2 = a2.borrow();
    let dx = a1.0 - a2.0;
    let dy = a1.1 - a2.1;
    [
        (a1.0 + dx, a1.1 + dy),
        (a2.0 - dx, a2.1 - dy)
    ]
}

fn all_antinodes(antennas: Vec<(i32, i32)>) -> impl Iterator<Item = (i32, i32)> {
    antennas.clone().into_iter()
        .flat_map(move |a1| antennas.clone().into_iter().map(move |a2| (a1, a2)))
        .filter(|(a1, a2)| a1 != a2)
        .flat_map(|(a1, a2)| antinodes(a1, a2).into_iter())
}

fn parse(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;
    for (x, line) in input.lines().filter(|line| line.len() > 0).enumerate() {
        rows = std::cmp::max(rows, x as i32 + 1);
        for (y, c) in line.chars().enumerate() {
            cols = std::cmp::max(cols, y as i32 + 1);
            if !c.is_ascii_alphanumeric() {
                continue;
            }
            antennas.entry(c).or_default().push((x as i32, y as i32));
        }
    }
    (antennas, rows, cols)
}


fn part_1(input: &str) -> usize {
    let (antennas, rows, cols) = parse(input);
    let antinode_set: HashSet<(i32, i32)> = antennas.into_iter()
        .flat_map(|(_, a)| all_antinodes(a))
        .filter(|&(x, y)| x >= 0 && x < rows && y >= 0 && y < cols)
        .collect();
    antinode_set.len()
}

use itertools::Itertools;

fn antinodes_pt_2<A: Borrow<(i32, i32)>>(a1: A, a2: A, rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let a1 = a1.borrow();
    let a2 = a2.borrow();
    let dx = a1.0 - a2.0;
    let dy = a1.1 - a2.1;
    let mut result = vec![];
    let mut x = a1.0;
    let mut y = a1.1;
    while x >= 0 && y >= 0 && x < rows && y < cols {
        result.push((x, y));
        x += dx;
        y += dy;
    }
    x = a2.0;
    y = a2.1;
    while x >= 0 && y >= 0 && x < rows && y < cols {
        result.push((x, y));
        x -= dx;
        y -= dy;
    }
    result
}

fn all_antinodes_pt_2(antennas: &[(i32, i32)], rows: i32, cols: i32) -> HashSet<(i32, i32)> {
    antennas.iter()
        .combinations(2)
        .flat_map(|combo| antinodes_pt_2(combo[0], combo[1], rows, cols))
        .collect()
}

fn part_2(input: &str) -> usize {
    let (antennas, rows, cols) = parse(input);
    antennas.values()
        .flat_map(|a_set| all_antinodes_pt_2(&a_set, rows, cols))
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
