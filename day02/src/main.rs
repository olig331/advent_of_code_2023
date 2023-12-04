use std::fs;

#[derive(Debug)]
struct GameParams {
    r: u32,
    g: u32,
    b: u32,
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("");

    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}

fn part_1(input: &String) -> u32 {
    let mut result: Vec<u32> = Vec::new();

    input
        .lines()
        .map(|x| x.split(":").nth(1).unwrap())
        .enumerate()
        .for_each(|(i, game)| {
            let mut passed = true;
            game.split(";").for_each(|bag| {
                let mut game_res = GameParams { r: 0, g: 0, b: 0 };
                bag.split(",").for_each(|item| {
                    let x = item.trim().split(" ").collect::<Vec<_>>();
                    match x[1] {
                        "red" => game_res.r += x[0].parse::<u32>().unwrap(),
                        "green" => game_res.g += x[0].parse::<u32>().unwrap(),
                        "blue" => game_res.b += x[0].parse::<u32>().unwrap(),
                        _ => print!("err"),
                    };
                });
                if game_res.r > 12 || game_res.g > 13 || game_res.b > 14 {
                    passed = false;
                }
            });
            if passed == true {
                result.push((i + 1) as u32);
            }
        });

    result.iter().sum::<u32>()
}

fn part_2(input: &String) -> u32 {
    let mut result: Vec<u32> = Vec::new();

    input
        .lines()
        .map(|x| x.split(":").nth(1).unwrap())
        .for_each(|game| {
            let mut game_res = GameParams { r: 0, g: 0, b: 0 };
            game.split(";").for_each(|bag| {
                bag.split(",").for_each(|item| {
                    let x = item.trim().split(" ").collect::<Vec<_>>();
                    let val = x[0].parse::<u32>().unwrap();
                    match x[1] {
                        "red" => {
                            if game_res.r < val {
                                game_res.r = val
                            }
                        }

                        "green" => {
                            if game_res.g < val {
                                game_res.g = val
                            }
                        }
                        "blue" => {
                            if game_res.b < val {
                                game_res.b = val
                            }
                        }
                        _ => print!("err"),
                    };
                });
            });
            result.push(game_res.r * game_res.g * game_res.b as u32);
        });

    result.iter().sum::<u32>()
}
