use regex::Regex;

fn part_1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).map(|caps| {
        let (_, [num_1, num_2]) = caps.extract();
        num_1.parse::<u64>().unwrap() * num_2.parse::<u64>().unwrap()
    }).sum()
}

fn part_2(input: &str) -> u64 {
    let enabled_re = Regex::new(r"(?s)(^|do\(\))(?<enabled>.*?)(don't\(\)|$)").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    enabled_re.captures_iter(input).flat_map(|enabled_caps| {
        let content = enabled_caps.name("enabled").unwrap().as_str();
        mul_re.captures_iter(content)
    }).map(|mul_caps| {
        let (_, [num_1, num_2]) = mul_caps.extract();
        num_1.parse::<u64>().unwrap() * num_2.parse::<u64>().unwrap()
    }).sum()
} 

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

