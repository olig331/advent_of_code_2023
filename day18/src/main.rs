use std::fs;

// https://www.wikiwand.com/en/Shoelace_formula shoelace algo i used
fn main() {
    let bind = fs::read_to_string("./input.txt").expect("");
    let input = bind
        .lines()
        .map(|l| {
            let val = l.split_whitespace().collect::<Vec<&str>>();
            (val[0], val[1].parse::<i32>().unwrap(), val[2])
        })
        .collect::<Vec<(&str, i32, &str)>>();

    let mut x = 0;
    let mut y = 0;
    let mut area = 2;

    for (dir, num, _) in input.clone() {
        let (prev_y, prev_x) = (y, x);
        match dir {
            "U" => y -= num,
            "D" => y += num,
            "L" => x -= num,
            "R" => x += num,
            _ => println!("Err"),
        };
        area += (x + prev_x) * (y - prev_y) + num;
    }

    println!("Part 1 - {}", area / 2);

    println!("Part 2 - {}", part_2(input.clone()));
}

fn part_2(input: Vec<(&str, i32, &str)>) -> i64 {
    let mut x = 0 as i64;
    let mut y = 0 as i64;
    let mut area = 2 as i64;

    for (_, _, hex) in input.clone() {
        let trim = hex.trim().replace("(", "").replace(")", "");
        let dir = match trim.trim().chars().last().unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => unreachable!(),
        };

        let h = trim.trim().split("").collect::<Vec<&str>>()[0..hex.len() - 2]
            .to_vec()
            .join("")
            .replace("#", "");

        let hex_val = i64::from_str_radix(&h, 16).unwrap();

        let (prev_y, prev_x) = (y, x);
        match dir {
            "U" => y -= hex_val,
            "D" => y += hex_val,
            "L" => x -= hex_val,
            "R" => x += hex_val,
            _ => println!("err"),
        };
        area += (x + prev_x) * (y - prev_y) + hex_val;
    }

    area / 2
}
