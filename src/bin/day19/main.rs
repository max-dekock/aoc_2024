#[derive(Debug)]
struct Puzzle {
    patterns: Vec<Vec<u8>>,
    designs: Vec<Vec<u8>>
}

fn parse_input(input: &str) -> Puzzle {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let patterns = first_line.split(", ").map(|token| token.as_bytes().to_vec()).collect();
    let designs = lines.filter(|line| line.len() > 0).map(|line| line.as_bytes().to_vec()).collect();
    Puzzle { patterns, designs }
}

fn is_design_possible(design: &[u8], patterns: &[Vec<u8>]) -> bool {
    let mut dp: Vec<bool> = vec![false; design.len() + 1];
    let mut pattern_seq: Vec<Vec<usize>> = vec![vec![]; design.len() + 1];
    dp[design.len()] = true;
    'i_loop: for i in (0..design.len()).rev() {
        let start_idx = patterns.partition_point(|pattern| pattern[0] < design[i]);
        let end_idx = patterns.partition_point(|pattern| pattern.as_slice() <= &design[i..]);
        for idx in start_idx..end_idx {
            let pattern = &patterns[idx];
            if pattern.len() <= design.len() - i &&
                dp[i + pattern.len()] &&
                design[i..].starts_with(pattern)
            {
                dp[i] = true;
                let mut seq = vec![idx];
                seq.extend_from_slice(&pattern_seq[i + pattern.len()]);
                pattern_seq[i] = seq;
                continue 'i_loop;
            }
        }
    }
    if dp[0] {
        let mut reconstructed: Vec<u8> = vec![];
        for &idx in &pattern_seq[0] {
            reconstructed.extend_from_slice(&patterns[idx]);
        }
        if reconstructed != design {
            println!("FALSE POSITIVE");
            println!("design        = {}", String::from_utf8_lossy(design));
            println!("reconstructed = {}", String::from_utf8_lossy(&reconstructed));
        }
    }
    dp[0]
}

fn part_1(input: &str) -> usize {
    let Puzzle { designs, mut patterns } = parse_input(input);
    patterns.sort();
    let mut count = 0;
    for design in &designs {
        if is_design_possible(design, &patterns) {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}
