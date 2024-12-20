fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.split_whitespace().map(|token| token.parse::<i32>().unwrap()).collect())
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }
    let first_diff = report[1] - report[0];
    if first_diff.abs() < 1 || first_diff.abs() > 3 {
        return false;
    }
    let diff_sign = first_diff.signum();
    for pair in report[1..].windows(2) {
        let diff = (pair[1] - pair[0]) * diff_sign;
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn part_1(input: &str) -> usize {
    let reports = parse(input);
    reports.into_iter().filter(|report| is_safe(&report)).count()
}

fn safe_with_dampener(report: &[i32]) -> bool {
    if is_safe(&report) {
        return true;
    }
    for i in 0..report.len() {
        let dampened_report: Vec<i32> = report.iter().take(i).chain(report.iter().skip(i+1)).copied().collect();
        if is_safe(&dampened_report) {
            return true;
        }
    }
    false        
}



fn part_2(input: &str) -> usize {
    let reports = parse(input);
    reports.into_iter().filter(|report| safe_with_dampener(&report)).count()
}


fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

