use std::collections::{HashSet, HashMap};

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

struct Puzzle {
    width: i32,
    height: i32,
    obstacles: HashSet<(i32, i32)>,
    starting_position: (i32, i32)
}

fn parse_input(input: &str) -> Puzzle {
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    let mut starting_position: Option<(i32, i32)> = None;
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let row_iter = input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            height += 1;
            width = std::cmp::max(width, line.len() as i32);
            line
        })
        .enumerate();
    let char_iter = row_iter
        .flat_map(|(x, line)| line.chars().enumerate().map(move |(y, c)| (x as i32, y as i32, c)));
    for (x, y, c) in char_iter {
        match c {
            '#' => { obstacles.insert((x, y)); },
            '^' => { starting_position.replace((x, y)); }
            _ => ()
        };
    }
    let starting_position = starting_position.expect("starting position not found");
    Puzzle {
        width,
        height,
        obstacles,
        starting_position
    }
}

fn part_1(input: &str) -> usize {
    let Puzzle { width, height, obstacles, starting_position } = parse_input(input);
    let mut position = starting_position;
    let mut direction_idx = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(starting_position);

    loop {
        let direction = DIRECTIONS[direction_idx];
        let next_x = position.0 + direction.0;
        let next_y = position.1 + direction.1;
        if obstacles.contains(&(next_x, next_y)) {
            direction_idx = (direction_idx + 1) % 4;
            continue;
        }
        if next_x < 0 || next_x >= height || next_y < 0 || next_y >= width {
            break;
        }
        position = (next_x, next_y);
        visited.insert(position);
    }

    return visited.len();
}

fn is_loop(input: &Puzzle) -> bool {
    let Puzzle { width, height, obstacles, starting_position } = input;
    let mut position = *starting_position;
    let mut direction_idx = 0;
    let mut visited: HashSet<((i32, i32), usize)> = HashSet::new();
    visited.insert((*starting_position, 0));

    loop {
        let direction = DIRECTIONS[direction_idx];
        let next_x = position.0 + direction.0;
        let next_y = position.1 + direction.1;
        if obstacles.contains(&(next_x, next_y)) {
            direction_idx = (direction_idx + 1) % 4;
            continue;
        }
        if next_x < 0 || next_x >= *height || next_y < 0 || next_y >= *width {
            break;
        }
        position = (next_x, next_y);
        if !visited.insert((position, direction_idx)) {
            return true;
        }
    }
    return false;
}

fn part_2(input: &str) -> usize {
    let mut puzzle = parse_input(input);
    let mut count = 0;
    for x in 0..puzzle.height {
        for y in 0..puzzle.width {
            if (x, y) == puzzle.starting_position {
                continue;
            }
            if puzzle.obstacles.contains(&(x, y)) {
                continue;
            }
            puzzle.obstacles.insert((x, y));
            if is_loop(&puzzle) {
                //println!("{}, {}", x, y);
                count += 1;
            }
            puzzle.obstacles.remove(&(x, y));
        }
    }
    count
}


/*fn part_2(input: &str) -> usize {
    let Puzzle { width, height, obstacles, starting_position } = parse_input(input);
    let mut position = starting_position;
    let mut direction_idx = 0;
    let mut visited: HashMap<(i32, i32), u8> = HashMap::new();
    let mut count = 0;

    loop {
        let direction = DIRECTIONS[direction_idx];
        let mut back_x = position.0 - direction.0;
        let mut back_y = position.1 - direction.1;
        while back_x > 0 && back_x <= height && back_y > 0 && back_y <= width && !obstacles.contains(&(back_x, back_y)) {
            *visited.entry((back_x, back_y)).or_default() |= 1 << direction_idx;
            back_x -= direction.0;
            back_y -= direction.1;
        }
        *visited.entry(position).or_default() |= 1 << direction_idx;
        let mut front_x = position.0 + direction.0;
        let mut front_y = position.1 + direction.1;
        while front_x > 0 && front_x <= height && front_y > 0 && front_y <= width && !obstacles.contains(&(front_x, front_y)) {
            let dirs = visited.entry((front_x, front_y)).or_default();
            if *dirs & (1 << ((direction_idx + 1) % 4)) != 0 {
                let obstacle_x = front_x + direction.0;
                let obstacle_y = front_y + direction.1;
                if (obstacle_x, obstacle_y) != starting_position
                    && obstacle_x >= 0
                    && obstacle_x < height
                    && obstacle_y >= 0
                    && obstacle_y < width
                    && !obstacles.contains(&(obstacle_x, obstacle_y))
                {
                    println!("{}, {}", front_x, front_y);
                    count += 1;
                }
            }
            *dirs |= 1 << direction_idx;
            position = (front_x, front_y);
            front_x += direction.0;
            front_y += direction.1;
        }

        if front_x < 0 || front_x >= height || front_y < 0 || front_y >= width {
            break;
        }
        direction_idx = (direction_idx + 1) % 4;
        //println!("{:?}", visited);
    }

    return count;
}*/

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
