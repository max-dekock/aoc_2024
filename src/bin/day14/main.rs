use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{space1, i32, line_ending},
    multi::separated_list1,
    sequence::{separated_pair, preceded}
};

use std::cmp::Ordering::*;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let vector = |label: &'static str| preceded(tag(label), separated_pair(i32, tag(","), i32));
    let (input, (position, velocity)) = separated_pair(vector("p="), space1, vector("v="))(input)?;
    Ok((input, Robot { position, velocity }))
}

fn parse_robot_list(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, parse_robot)(input)
}

fn simulate_axis(start: i32, vel: i32, t: i32, m: i32) -> i32 {
    let mut end = ((start % m) + ((vel % m) * (t % m)) % m) % m;
    if end < 0 {
        end += m;
    }
    end
}

fn simulate_robot(robot: &Robot, seconds: i32, bathroom_size: (i32, i32)) -> (i32, i32) {
    let x = simulate_axis(robot.position.0, robot.velocity.0, seconds, bathroom_size.0);
    let y = simulate_axis(robot.position.1, robot.velocity.1, seconds, bathroom_size.1);
    (x, y)
}

fn quadrant(position: (i32, i32), bathroom_size: (i32, i32)) -> Option<usize> {
    let midline = (bathroom_size.0 / 2, bathroom_size.1 / 2);
    match (position.0.cmp(&midline.0), position.1.cmp(&midline.1)) {
        (Less, Less) => Some(0),
        (Less, Greater) => Some(1),
        (Greater, Less) => Some(2),
        (Greater, Greater) => Some(3),
        _ => None
    }
}

fn part_1(input: &str) -> usize {
    let bathroom_size = (101, 103);
    //let bathroom_size = (11, 7);
    let seconds = 100;
    let mut quadrant_counts = [0; 4];
    let (_, robots) = parse_robot_list(input).unwrap();
    for robot in &robots {
        let new_position = simulate_robot(robot, seconds, bathroom_size);
        if let Some(q) = quadrant(new_position, bathroom_size) {
            quadrant_counts[q] += 1;
        }
    }
    quadrant_counts.into_iter().product()
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}
