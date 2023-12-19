use rayon::prelude::*;
use std::fs;

fn main() {
    let binding = fs::read_to_string("./input.txt").expect("");
    let input: Vec<&str> = binding.split("\n\n").collect();
    let seeds = input[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let maps = input[1..]
        .to_vec()
        .iter_mut()
        .map(|m| {
            m.lines().collect::<Vec<&str>>()[1..]
                .to_vec()
                .iter()
                .map(|s| {
                    s.split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect::<Vec<Vec<Vec<u64>>>>();

    let mut part_1_lowest = std::u64::MAX;
    let s_clone_p1 = seeds.clone();
    s_clone_p1.into_iter().for_each(|seed| {
        let mut result = seed;
        for m in &maps {
            for r in m {
                if result >= r[1] && result <= r[1] + r[2] {
                    result = r[0] + (result - r[1]).abs_diff(0);
                    break;
                }
            }
        }
        if result < part_1_lowest {
            part_1_lowest = result;
        }
    });

    let s_clone_p2 = seeds.clone();
    let part_2_lowest = s_clone_p2
        .chunks(2)
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<u64>>>()
        .par_iter()
        .map(|sr| {
            let mut local_lowest: u64 = std::u64::MAX;

            for i in 1..sr[1] {
                let mut result = sr[0] + i;

                for m in &maps {
                    for r in m {
                        if result >= r[1] && result <= r[1] + r[2] {
                            result = r[0] + (result - r[1]).abs_diff(0);
                            break;
                        }
                    }
                }

                if result < local_lowest {
                    local_lowest = result;
                }
            }

            local_lowest
        })
        .min()
        .unwrap();

    println!("Part 1 - {}", part_1_lowest);
    println!("Part 2 - {}", part_2_lowest);
}
