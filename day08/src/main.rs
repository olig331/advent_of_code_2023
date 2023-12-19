use std::collections::BTreeMap;
use std::fs;

fn main() {
    let binding = fs::read_to_string("./input.txt").expect("");
    let input: Vec<&str> = binding.split("\n\n").collect();

    let mut d: BTreeMap<&str, (&str, &str)> = BTreeMap::new();

    if let Some(lines) = input.iter().nth(1) {
        for line in lines.lines() {
            let parts: Vec<&str> = line.trim().split(" = ").collect();
            let vals: Vec<&str> = parts[1].split(',').map(|i| i.trim()).collect();

            let y: Vec<&str> = vals
                .iter()
                .map(|i| i.trim_matches(|c| c == '(' || c == ')'))
                .collect();

            d.insert(parts[0], (&y[0], &y[1]));
        }
    }

    let mut result = 0;
    let mut key = "AAA";
    'out: loop {
        for t in input[0].chars() {
            let item = d.get_mut(key).unwrap();
            if t == 'L' {
                result += 1;
                if item.0 == "ZZZ" {
                    break 'out;
                }
                key = item.0;
            } else {
                result += 1;
                if item.1 == "ZZZ" {
                    break 'out;
                }
                key = item.1;
            }
        }
    }

    println!("{:?}", result);
    part_2(d, input);
}

fn part_2(mut tree: BTreeMap<&str, (&str, &str)>, input: Vec<&str>) {
    let mut starting_nodes: Vec<&str> = Vec::new();
    for (key, _) in tree.iter() {
        if key.ends_with("A") {
            starting_nodes.push(key)
        }
    }

    let mut node_results: Vec<u64> = Vec::new();

    for node in starting_nodes {
        let mut key = node;
        let mut result: u64 = 0;
        'out: loop {
            for t in input[0].chars() {
                let item = tree.get_mut(key).unwrap();
                if t == 'L' {
                    result += 1;
                    if item.0.ends_with("Z") {
                        node_results.push(result);
                        break 'out;
                    }
                    key = item.0;
                } else {
                    result += 1;
                    if item.1.ends_with("Z") {
                        node_results.push(result);
                        break 'out;
                    }
                    key = item.1;
                }
            }
        }
    }

    let mut part_2_result = 1;

    for &res in node_results.iter() {
        part_2_result = (part_2_result * res) / greatest_common_divisor(part_2_result, res)
    }

    println!("{:?}", part_2_result)
}

fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
