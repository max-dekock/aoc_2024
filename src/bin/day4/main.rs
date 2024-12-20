const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn word_search(grid: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    for (i, c) in XMAS.iter().enumerate() {
        let x_prime_opt = x.checked_add_signed(dx * i as isize);
        let y_prime_opt = y.checked_add_signed(dy * i as isize);
        let coord_opt = x_prime_opt.zip(y_prime_opt);
        let char_opt = coord_opt.and_then(|(x_prime, y_prime)| grid.get(x_prime).and_then(|row| row.get(y_prime)));
        if !char_opt.is_some_and(|cc| c == cc) {
            return false;
        }
    }
    return true;
}

fn part_1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for x in 0..rows {
        for y in 0..cols {
            for dx in [-1isize, 0isize, 1isize] {
                for dy in [-1isize, 0isize, 1isize] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if word_search(&grid, x, y, dx, dy) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn part_2(input: &str) -> u32 {
    let mut count = 0;
    let grid: Vec<Vec<char>> = input.lines().filter(|line| line.len() > 0).map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    for x in 1..rows-1 {
        for y in 1..cols-1 {
            if grid[x][y] == 'A' {
                let ul = grid[x-1][y-1];
                let ur = grid[x-1][y+1];
                let dl = grid[x+1][y-1];
                let dr = grid[x+1][y+1];
                let pos_diag = (dl == 'M' && ur == 'S') || (dl == 'S' && ur == 'M');
                let neg_diag = (ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M');
                if pos_diag && neg_diag {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
