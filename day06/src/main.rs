use std::fs;

fn main() {
    let input: Vec<Vec<u32>> = fs::read_to_string("./input.txt")
        .expect("")
        .lines()
        .map(|l| {
            l.split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let (time, distance): (Vec<u32>, Vec<u32>) = (input[0].clone(), input[1].clone());

    let mut result: Vec<u32> = Vec::new();

    for i in 0..time.len() {
        let mut race_res: u32 = 0;

        let d = distance[i];
        let t = time[i];

        for j in 0..=t {
            let tr = t - j;

            let dc = j * tr;
            if dc > d {
                race_res += 1;
            }
        }
        if race_res > 0 {
            result.push(race_res);
        }
    }

    println!("Part 1 - {}", result.iter().fold(1, |acc, &x| acc * x));

    part_2()
}

fn part_2() {
    let (time, distance) = (54946592, 302147610291404 as u64);
    let mut race_res: u64 = 0;

    let d = distance;
    let t = time;

    for j in 0..=t {
        let tr = t - j;

        let dc = j * tr;
        if dc > d {
            race_res += 1;
        }
    }
    println!("Part 2 - {}", race_res);
}
