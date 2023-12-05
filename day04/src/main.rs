use std::fs;

#[derive(Debug)]
struct Game {
    amount: u32,
}

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let (p1, p2) = process(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn process(input: &Vec<String>) -> (u32, u32) {
    let mut part_1_result: u32 = 0;
    let mut vec: Vec<Game> = Vec::new();

    input.iter().for_each(|_g| vec.push(Game { amount: 1 }));

    input.iter().enumerate().for_each(|(i, l)| {
        let binding = l.split(":").nth(1).unwrap_or("").trim().to_string();
        let game = binding.split(" | ").collect::<Vec<&str>>();

        let count = parse_array(game[1])
            .iter()
            .filter(|&item| parse_array(game[0]).contains(item))
            .count();

        let mut score = if count > 0 { 1 } else { 0 };
        for _ in 1..count {
            score *= 2
        }

        for j in 1..=count {
            vec[i + j].amount += vec[i].amount
        }

        part_1_result += score
    });

    let prod: Vec<u32> = vec.iter().map(|g| return g.amount).collect();
    let part_2_result = prod.iter().sum::<u32>();

    (part_1_result, part_2_result)
}

fn parse_array(input: &str) -> Vec<u32> {
    let result = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    result
}
