use nom::{
    IResult,
    error::ParseError,
    bytes::complete::tag,
    character::complete::{i32, space1, multispace1, line_ending},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, tuple}
};

#[derive(Debug)]
struct Machine {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32)
}

fn parse_machine_parameter(label: &str, op: &str) -> impl FnMut(&str) -> IResult<&str, (i32, i32)>{
    preceded(
        tuple((tag(label), tag(":"), space1)),
        separated_pair(
            preceded(pair(tag("X"), tag(op)), i32),
            pair(tag(","), space1),
            preceded(pair(tag("Y"), tag(op)), i32),
        )
    )
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, button_a) = parse_machine_parameter("Button A", "+")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, button_b) = parse_machine_parameter("Button B", "+")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, prize) = parse_machine_parameter("Prize", "=")(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Machine { button_a, button_b, prize }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(multispace1, parse_machine)(input)
}

fn part_1(input: &str) -> u32 {    
    let machines = parse_input(input).unwrap();
    println!("{:?}", machines);
    0
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}
