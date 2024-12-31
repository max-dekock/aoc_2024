use aoc_2024::coord::Coord;
use aoc_2024::grid::Grid;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    multi::separated_list1,
    sequence::separated_pair
};
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::cmp::Reverse;

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    let (input, (x, y)) = separated_pair(i64, tag(","), i64)(input)?;
    Ok((input, Coord(x, y)))
}

fn parse_coord_list(input: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(line_ending, parse_coord)(input)
}

fn generate_grid(obstacles: &[Coord], width: usize, height: usize) -> Grid<bool> {
    let mut grid = Grid::new_with_default(width, height);
    for &obstacle in obstacles {
        if let Some(cell) = grid.get_mut(obstacle) {
            *cell = true;
        }
    }
    grid
}

fn taxicab_distance(a: Coord, b: Coord) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

const DIRECTIONS: [Coord; 4] = [
    Coord(0, 1),
    Coord(0, -1),
    Coord(1, 0),
    Coord(-1, 0)
];

fn a_star(grid: &Grid<bool>, start: Coord, end: Coord) -> Option<Vec<Coord>> {
    let mut pq: PriorityQueue<Coord, Reverse<i64>> = PriorityQueue::new();
    let mut g_score: HashMap<Coord, i64> = HashMap::new();
    let mut preceding: HashMap<Coord, Coord> = HashMap::new();
    pq.push(start, Reverse(taxicab_distance(start, end)));
    g_score.insert(start, 0);

    while let Some((coord, _)) = pq.pop() {
        if coord == end {
            return Some(reconstruct_path(&preceding, coord));
        }
        let g = *g_score.get(&coord).unwrap();
        for direction in DIRECTIONS {
            let neighbor = coord + direction;
            if grid.get(neighbor).is_none_or(|&cell| cell == true) {
                continue;
            }
            if g_score.get(&neighbor).is_none_or(|&old_score| old_score > g + 1) {
                preceding.insert(neighbor, coord);
                g_score.insert(neighbor, g + 1);
                pq.push(neighbor, Reverse(g + 1 + taxicab_distance(neighbor, end)));
            }
        }
    }

    None
}

fn reconstruct_path(preceding: &HashMap<Coord, Coord>, end: Coord) -> Vec<Coord> {
    let mut path = vec![end];
    let mut coord = end;
    while let Some(&prev) = preceding.get(&coord) {
        path.push(prev);
        coord = prev;
    }
    path.reverse();
    path
}

fn part_1(input: &str) -> usize {
    let mut obstacle_list = parse_coord_list(input).unwrap().1;
    obstacle_list.truncate(1024);
    let grid = generate_grid(&obstacle_list, 71, 71);
    let shortest_path = a_star(&grid, Coord(0, 0), Coord(70, 70)).unwrap();
    //println!("{:?}", shortest_path);
    shortest_path.len() - 1
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}
