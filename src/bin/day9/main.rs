fn part_1(input: &str) -> u64 {
    let mut id = 0;
    let mut disk: Vec<Option<u64>> = vec![];
    for chunk in input.trim_end().as_bytes().chunks(2) {
        if let Some((file_size, free_space)) = match chunk {
            [c1, c2] => {
                if !c1.is_ascii_digit() {
                    panic!("invalid character in non-terminal position")
                } else if !c2.is_ascii_digit() {
                    Some((c1 - b'0', 0))
                } else {
                    Some((c1 - b'0', c2 - b'0'))
                }
            },
            [c1] => {
                if !c1.is_ascii_digit() {
                    None
                } else {
                    Some((c1 - b'0', 0))
                }
            },
            _ => None
        } {
            for _ in 0..file_size {
                disk.push(Some(id));
            }
            for _ in 0..free_space {
                disk.push(None);
            }
            id += 1;
        }
    }
    let mut idx = 0;
    let mut new_disk = vec![];
    while idx < disk.len() {
        if let Some(id) = disk[idx] {
            new_disk.push(id);
        } else {
            if let Some(Some(id)) = disk.pop() {
                new_disk.push(id);
            } else {
                // end of disk is always Some
                unreachable!();
            }
            while let Some(None) = disk.last() {
                // maintain invariant
                disk.pop();
            }
        }
        idx += 1;
    }
    compute_checksum(&new_disk)
}

fn compute_checksum(disk: &[u64]) -> u64 {
    let mut checksum = 0;
    for i in 0..disk.len() {
        checksum += i as u64 * disk[i];
    }
    checksum
}

use std::collections::HashMap;

fn part_2(input: &str) -> u64 {
    let mut files: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut spaces: Vec<(u64, u64)> = Vec::new();
    let mut id = 0;
    let mut idx = 0;
    input
        .trim_end()
        .as_bytes()
        .chunks(2)
        .for_each(|chunk| match chunk {
            [c1, c2] => {
                if !c1.is_ascii_digit() || !c2.is_ascii_digit() {
                    panic!("invalid character");
                }
                let file_size = (c1 - b'0') as u64;
                let space_size = (c2 - b'0') as u64;
                files.insert(id, (idx, file_size));
                idx += file_size;
                spaces.push((idx, space_size));
                idx += space_size;
                id += 1;
            },
            [c1] => {
                if !c1.is_ascii_digit() {
                    panic!("invalid character");
                }
                files.insert(id, (idx, (c1 - b'0') as u64));
            },
            _ => ()
        });

    for id in (1..=id).rev() {
        let (file_pos, file_size) = files.get_mut(&id).unwrap();
        for (space_pos, space_size) in &mut spaces {
            if *space_pos > *file_pos {
                break;
            }
            if *space_size < *file_size {
                continue;
            }
            *file_pos = *space_pos;
            *space_pos += *file_size;
            *space_size -= *file_size;
        }
    }

    let mut checksum = 0;
    for (id, (file_idx, size)) in files {
        for idx in file_idx..file_idx+size {
            checksum += id * idx;
        }
    }
    checksum
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
