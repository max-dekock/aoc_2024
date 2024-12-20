use std::collections::{HashSet, HashMap};

fn score_trailhead(grid: &Vec<Vec<u8>>, head: (i32, i32), rows: i32, cols: i32) -> u32 {
    let mut stack = vec![head];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_nines: HashSet<(i32, i32)> = HashSet::new();
    while let Some((x, y)) = stack.pop() {
        if !visited.insert((x, y)) {
            continue;
        }
        let height = grid[x as usize][y as usize];
        if height == 9 {
            visited_nines.insert((x, y));
            continue;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x2 = x + dx;
            let y2 = y + dy;
            if x2 < 0 || x2 >= rows || y2 < 0 || y2 >= cols {
                continue;
            }
            if grid[x2 as usize][y2 as usize] != height + 1 {
                continue;
            }
            stack.push((x2, y2));
        }
    }
    visited_nines.len() as u32
}

fn part_1(input: &str) -> u32 {
    let mut zeros: HashSet<(i32, i32)> = HashSet::new();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    for (x, line) in input.lines().enumerate() {
        let x = x as i32;
        if line.len() == 0 {
            continue;
        }
        let mut row = Vec::new();
        for (y, c) in line.chars().enumerate() {
            let y = y as i32;
            let height = c.to_digit(10).unwrap() as u8;
            if height == 0 {
                zeros.insert((x, y));
            }
            row.push(height);
            cols = std::cmp::max(cols, y + 1);
        }
        grid.push(row);
        rows = std::cmp::max(rows, x + 1);
    }
    let mut total_score = 0;
    for zero in zeros {
        total_score += score_trailhead(&grid, zero, rows, cols);
    }
    return total_score;
}

fn part_2(input: &str) -> u32 {
    let mut ways_by_height: Vec<HashMap<(i32, i32), u32>> = vec![HashMap::new()];
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    for (x, line) in input.lines().enumerate() {
        let x = x as i32;
        if line.len() == 0 {
            continue;
        }
        let mut row = Vec::new();
        for (y, c) in line.chars().enumerate() {
            let y = y as i32;
            let height = c.to_digit(10).unwrap() as u8;
            if height == 0 {
                ways_by_height[0].insert((x, y), 1);
            }
            row.push(height);
            cols = std::cmp::max(cols, y + 1);
        }
        grid.push(row);
        rows = std::cmp::max(rows, x + 1);
    }
    for h in 1..=9 {
        let mut ways: HashMap<(i32, i32), u32> = HashMap::new();
        for ((x, y), n) in &ways_by_height[h-1] {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let x2 = x + dx;
                let y2 = y + dy;
                if x2 < 0 || x2 >= rows || y2 < 0 || y2 >= cols {
                    continue;
                }
                if grid[x2 as usize][y2 as usize] as usize != h {
                    continue;
                }
                *ways.entry((x2, y2)).or_insert(0) += n;
            }
        }
        ways_by_height.push(ways);
    }
    ways_by_height[9].values().sum()
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
