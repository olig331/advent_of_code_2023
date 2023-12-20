use std::fs;

fn main() {
    let binding = fs::read_to_string("./input.txt").expect("");
    let input = binding
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut part_1_total = 0;
    for item in input.clone() {
        let mut result_arr: Vec<Vec<i64>> = Vec::new();

        let val = item.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
        result_arr.push(val);

        while !result_arr
            .last()
            .unwrap()
            .iter()
            .all(|&x| x == result_arr.last().unwrap()[0])
        {
            result_arr.push(
                result_arr
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<i64>>(),
            );
        }
        let mut val = 0;
        for (i, l) in result_arr.iter_mut().rev().enumerate() {
            if i != 0 {
                l.push(l.last().unwrap() + val)
            }
            val = l.last().unwrap().clone();
        }

        part_1_total += item.last().unwrap() + result_arr.first().unwrap().last().unwrap();
    }

    let mut part_2_total = 0;
    for item in input.clone() {
        let mut result_arr: Vec<Vec<i64>> = Vec::new();

        let rev: Vec<&i64> = item.iter().rev().collect();

        let val = rev.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
        result_arr.push(val);

        while !result_arr
            .last()
            .unwrap()
            .iter()
            .all(|&x| x == result_arr.last().unwrap()[0])
        {
            result_arr.push(
                result_arr
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<i64>>(),
            );
        }
        let mut val = 0;
        for (i, l) in result_arr.iter_mut().rev().enumerate() {
            if i != 0 {
                l.push(l.last().unwrap() + val)
            }
            val = l.last().unwrap().clone();
        }

        part_2_total += item.first().unwrap() + result_arr.first().unwrap().last().unwrap();
    }

    println!("{}", part_1_total);
    println!("{}", part_2_total)
}
