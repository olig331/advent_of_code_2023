use std::{collections::HashMap, fs};

use helpers::{gen_indices, get_slices, has_symbol, insert_to_map};

mod helpers;

fn part_1(grid: &Vec<String>) -> u64 {
    let mut res: Vec<u64> = Vec::new();

    grid.iter().enumerate().for_each(|(y, line)| {
        let items = helpers::gen_indices(line);

        items.iter().for_each(|item| {
            let mut x_start_val = item.start;
            let mut x_end_val = item.end;

            if item.start > 0 {
                x_start_val -= 1;
            }

            if item.end < line.len() {
                x_end_val += 1;
            }

            let (up_slice, down_slice, right_slice, left_slice) =
                get_slices(grid, y, x_start_val, x_end_val, item);

            if has_symbol(&up_slice)
                || has_symbol(&down_slice)
                || has_symbol(&left_slice)
                || has_symbol(&right_slice)
            {
                res.push(item.val);
            }
        });
    });

    res.iter().sum::<u64>()
}

fn has_gear(input: &str) -> bool {
    input.contains('*')
}

fn part_2(grid: &Vec<String>) -> u64 {
    let mut map: HashMap<String, Vec<u64>> = HashMap::new();

    grid.iter().enumerate().for_each(|(y, line)| {
        let items = gen_indices(line);

        items.iter().for_each(|item| {
            let mut x_start_val = item.start;
            let mut x_end_val = item.end;
            let mut gear_x = item.start;

            if item.start > 0 {
                x_start_val -= 1;
                gear_x -= 1;
            }

            if item.end < line.len() {
                x_end_val += 1;
            }

            let (up_slice, down_slice, right_slice, left_slice) =
                get_slices(grid, y, x_start_val, x_end_val, item);

            if has_gear(&up_slice) {
                let index = up_slice.find('*').unwrap() as u32;
                insert_to_map(
                    &mut map,
                    format!("{}-{}", y - 1, (gear_x as u32 + index)),
                    item.val,
                )
            }
            if has_gear(&down_slice) {
                let index = down_slice.find('*').unwrap() as u32;
                insert_to_map(
                    &mut map,
                    format!("{}-{}", y + 1, (gear_x as u32 + index)),
                    item.val,
                )
            }
            if has_gear(&left_slice) {
                insert_to_map(&mut map, format!("{}-{}", y, item.start - 1), item.val)
            }
            if has_gear(&right_slice) {
                insert_to_map(&mut map, format!("{}-{}", y, item.end), item.val)
            }
        });
    });

    let gear_products: HashMap<String, u64> = map
        .into_iter()
        .filter(|(_, values)| values.len() > 1)
        .map(|(key, values)| (key.clone(), values.iter().copied().product()))
        .collect();

    gear_products.values().sum::<u64>()
}

fn main() {
    let grid: Vec<String> = fs::read_to_string("./input.txt")
        .expect("")
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(&grid));
}
