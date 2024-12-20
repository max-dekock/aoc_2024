use std::collections::HashMap;

fn part_1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input.lines().filter(|line| line.len() > 0).map(|line| {
        let nums: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        assert!(nums.len() == 2);
        (nums[0], nums[1])
    }).unzip();
    left.sort();
    right.sort();
    left.into_iter().zip(right.into_iter()).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part_2(input: &str) -> u32 {
    let (left, right): (Vec<u32>, Vec<u32>) = input.lines().filter(|line| line.len() > 0).map(|line| {
        let nums: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        assert!(nums.len() == 2);
        (nums[0], nums[1])
    }).unzip();
    let mut right_count: HashMap<u32, u32> = HashMap::new();
    for num in right {
        right_count.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut similarity = 0;
    for num in left {
        similarity += num * right_count.get(&num).unwrap_or(&0);
    }
    return similarity;
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
    
}
