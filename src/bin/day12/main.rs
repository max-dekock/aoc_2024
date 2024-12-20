use std::collections::{BTreeSet, BTreeMap};

fn part_1(input: &str) -> u32 {
    let mut rows = 0;
    let mut cols = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut unvisited: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (x, line) in input.lines().filter(|line| line.len() > 0).enumerate() {
        let x = x as i32;
        rows = std::cmp::max(x + 1, rows);
        let mut row = Vec::new();
        for (y, c) in line.chars().enumerate() {
            let y = y as i32;
            cols = std::cmp::max(y + 1, cols);
            row.push(c);
            unvisited.insert((x, y));
        }
        grid.push(row);
    }

    let mut price = 0;

    while let Some((x, y)) = unvisited.pop_first() {
        let plant_type = grid[x as usize][y as usize];
        let mut stack: Vec<(i32, i32)> = vec![(x, y)];
        let mut perimeter = 0;
        let mut area = 0;
        while let Some((x, y)) = stack.pop() {
            area += 1;
            for (u, v) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)] {
                if u < 0 || u >= rows || v < 0 || v >= cols {
                    perimeter += 1;
                    continue;
                }
                if grid[u as usize][v as usize] == plant_type {
                    if unvisited.contains(&(u, v)) {
                        unvisited.remove(&(u, v));
                        stack.push((u, v));
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        //println!("region of type {} has area {} and perimeter {}", plant_type, area, perimeter);
        //println!("price of region: {}", perimeter * area);
        price += perimeter * area;
    }
    price
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn perpendicular_dirs(dir: usize) -> [usize; 2] {
    if dir == 0 || dir == 1 {
        [2, 3]
    } else if dir == 2 || dir == 3 {
        [0, 1]
    } else {
        unreachable!()
    }
}

fn part_2(input: &str) -> i32 {
    let mut rows = 0;
    let mut cols = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut unvisited: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (x, line) in input.lines().filter(|line| line.len() > 0).enumerate() {
        let x = x as i32;
        rows = std::cmp::max(x + 1, rows);
        let mut row = Vec::new();
        for (y, c) in line.chars().enumerate() {
            let y = y as i32;
            cols = std::cmp::max(y + 1, cols);
            row.push(c);
            unvisited.insert((x, y));
        }
        grid.push(row);
    }

    let mut price = 0;

    while let Some((x, y)) = unvisited.pop_first() {
        let plant_type = grid[x as usize][y as usize];
        let mut stack: Vec<(i32, i32)> = vec![(x, y)];
        let mut side_count = 0;
        let mut area = 0;
        let mut side_map: BTreeMap<(i32, i32), [bool; 4]> = BTreeMap::new();
        while let Some((x, y)) = stack.pop() {
            area += 1;
            let mut sides = [false; 4];
            let mut neighbor_side_count = [0; 4];
            for dir_idx in 0..4 {
                let (dx, dy) = DIRS[dir_idx];
                let (u, v) = (x + dx, y + dy);
                if u < 0 || v < 0 || u >= rows || v >= cols {
                    sides[dir_idx] = true;
                    continue;
                }
                if grid[u as usize][v as usize] != plant_type {
                    sides[dir_idx] = true;
                    continue;
                }
                if unvisited.contains(&(u, v)) {
                    unvisited.remove(&(u, v));
                    stack.push((u, v));
                    continue;
                }
                if let Some(neighbor_sides) = side_map.get(&(u, v)) {
                    for d in perpendicular_dirs(dir_idx) {
                        if neighbor_sides[d] {
                            neighbor_side_count[d] += 1;
                        }
                    }
                }
            }
            for dir_idx in 0..4 {
                if sides[dir_idx] {
                    side_count += 1 - neighbor_side_count[dir_idx];
                }
            }
            side_map.insert((x, y), sides);
        }
        //println!("region of type {} has area {} and {} sides", plant_type, area, side_count);
        //println!("price of region: {}", area * side_count);
        price += area * side_count;
    }
    price
}



fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
