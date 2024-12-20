use regex::Regex;

struct Puzzle {
    ordering_rules: Vec<(u32, u32)>,
    pages_to_produce: Vec<Vec<u32>>
}


fn parse(input: &str) -> Puzzle {
    let mut ordering_rules: Vec<(u32, u32)> = Vec::new();
    let mut pages_to_produce: Vec<Vec<u32>> = Vec::new();
    let lines = input.lines()
        .filter(|line| line.len() > 0);
    let ordering_re = Regex::new(r"^([0-9]+)\|([0-9]+)$").unwrap();
    for line in lines {
        if let Some(caps) = ordering_re.captures(line) {
            let left = caps[1].parse::<u32>().unwrap();
            let right = caps[2].parse::<u32>().unwrap();
            ordering_rules.push((left, right));
        } else {
            let update = line.split(',').map(|token| token.parse::<u32>().unwrap()).collect();
            pages_to_produce.push(update);
        }
    }
    Puzzle {
        ordering_rules,
        pages_to_produce
    }
}

fn update_in_right_order(ordering_rules: &[(u32, u32)], update: &[u32]) -> bool {
    for (pre, post) in ordering_rules {
        let pre_idx_opt = update.iter().position(|page| page == pre);
        let post_idx_opt = update.iter().position(|page| page == post);
        if let Some((pre_idx, post_idx)) = pre_idx_opt.zip(post_idx_opt) {
            if pre_idx > post_idx {
                return false;
            }
        }
    }
    true
}

fn part_1(input: &str) -> u32 {
    let Puzzle { ordering_rules, pages_to_produce } = parse(input);
    let mut result = 0;
    for update in pages_to_produce {
        if update_in_right_order(&ordering_rules, &update) {
            result += update[update.len() / 2];
        }
    }
    return result;
}

use std::collections::{HashSet, HashMap, VecDeque};

fn topo_sort(edge_list: &[(u32, u32)]) -> Vec<u32> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    for &(from, to) in edge_list {
        graph.entry(from).or_default().push(to);
        in_degree.entry(to).and_modify(|d| *d += 1).or_insert(1);
        in_degree.entry(from).or_insert(0);
    }

    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter_map(|(&node, &deg)| if deg == 0 { Some(node) } else { None })
        .collect();

    let mut sorted: Vec<u32> = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    if sorted.len() == in_degree.len() {
        sorted
    } else {
        panic!("cycle detected");
    }
}

fn reorder_pages(pages: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let pages: HashSet<u32> = pages.iter().copied().collect();
    let rules_filtered: Vec<(u32, u32)> = rules.iter()
        .filter(|(pre, post)| pages.contains(pre) && pages.contains(post))
        .copied()
        .collect();
    let pages_sorted = topo_sort(&rules_filtered);
    pages_sorted.into_iter().filter(|page| pages.contains(page)).collect()
}

fn part_2(input: &str) -> u32 {
    let Puzzle { ordering_rules, pages_to_produce } = parse(input);
    pages_to_produce.iter()
        .filter(|pages| !update_in_right_order(&ordering_rules, pages))
        .map(|pages| {
            let reordered = reorder_pages(pages, &ordering_rules);
            reordered[reordered.len() / 2]
        })
        .sum()
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
