use aoc_2024::coord::Coord;
use aoc_2024::grid::Grid;
use std::fmt;

fn move_from_char(c: char) -> Option<Coord> {
    match c {
        '<' => Some(Coord(-1, 0)),
        '>' => Some(Coord(1, 0)),
        '^' => Some(Coord(0, -1)),
        'v' => Some(Coord(0, 1)),
        _ => None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SokobanCell {
    Wall,
    Boulder,
    Robot,
    Empty
}

use SokobanCell::*;

impl TryFrom<char> for SokobanCell {
    type Error = char;

    fn try_from(value: char) -> Result<SokobanCell, char> {
        match value {
            '#' => Ok(Wall),
            'O' => Ok(Boulder),
            '@' => Ok(Robot),
            '.' => Ok(Empty),
            c => Err(c)
        }
    }
}

impl fmt::Display for SokobanCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Wall => '#',
            Boulder => 'O',
            Robot => '@',
            Empty => '.'
        };
        write!(f, "{c}")
    }
}

fn apply_move(
    direction: Coord,
    grid: &mut Grid<SokobanCell>,
    robot_coord: &mut Coord
) {
    let new_robot_coord = *robot_coord + direction;
    let new_robot_cell = grid.get(new_robot_coord);
    let move_ok = match new_robot_cell {
        Some(Empty) => true,
        Some(Boulder) => shift_boulder(direction, grid, new_robot_coord),
        _ => false
    };
    if move_ok {
        *grid.get_mut(*robot_coord).unwrap() = Empty;
        *grid.get_mut(new_robot_coord).unwrap() = Robot;
        *robot_coord = new_robot_coord;
    }
}

fn shift_boulder(
    direction: Coord,
    grid: &mut Grid<SokobanCell>,
    boulder_coord: Coord
) -> bool {
    let new_coord = boulder_coord + direction;
    let shift_ok = match grid.get(new_coord) {
        Some(Empty) => true,
        Some(Boulder) => shift_boulder(direction, grid, new_coord),
        _ => false
    };
    if shift_ok {
        *grid.get_mut(boulder_coord).unwrap() = Empty;
        *grid.get_mut(new_coord).unwrap() = Boulder;
    }
    shift_ok
}

fn gps_coordinate(coord: Coord) -> i64 {
    coord.0 + 100 * coord.1
}

fn part_1(input: &str) -> i64 {
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut grid: Grid<SokobanCell> = Grid::try_from_rows(
        grid_str.lines()
            .map(|line| line.chars().map(|c| c.try_into().unwrap()))
    ).unwrap();
    let moves: Vec<Coord> = moves_str.chars().flat_map(|c| move_from_char(c)).collect();
    let mut robot_coord = grid.iter_with_coords()
        .find(|&(_, &cell)| cell == Robot)
        .unwrap().0;
    for direction in moves {
        apply_move(direction, &mut grid, &mut robot_coord);
        //println!("{}", grid);
    }

    grid.iter_with_coords()
        .filter(|&(_, &cell)| cell == Boulder)
        .map(|(coord, _)| gps_coordinate(coord))
        .sum()
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}
