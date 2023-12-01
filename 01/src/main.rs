use std::fs;

fn main() {
    println!(
        "Part 1: {:?}",
        fs::read_to_string("./input.txt")
            .expect("")
            .lines()
            .map(|x| {
                x.chars()
                    .filter_map(|c| c.to_string().parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            })
            .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
            .sum::<u32>()
    );

    println!(
        "Part 2: {:?}",
        fs::read_to_string("./input.txt")
            .expect("")
            .lines()
            .map(|x| {
                x.replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
                    .chars()
                    .filter_map(|c| c.to_string().parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            })
            .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
            .sum::<u32>()
    );
}
