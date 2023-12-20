use std::{fs, usize, vec};

type Coords = (usize, usize);

fn heuristic_manhattan(a: &Coords, b: &Coords) -> u64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u64
}

fn main() {
    let binding = fs::read_to_string("./input.txt").expect("");
    let mut gal = Vec::new();

    let mut expand_indexes: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());

    let input_as_vec = binding
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (y, l) in input_as_vec.clone().iter().enumerate() {
        if l.iter().all(|&c| c == '.') {
            expand_indexes.0.push(y);
        }

        l.iter().enumerate().for_each(|(x, c)| {
            if c == &'#' {
                gal.push((y, x))
            }
        })
    }

    for x in 0..input_as_vec[0].iter().len() {
        let mut all = true;
        for i in 0..input_as_vec.len() {
            if input_as_vec[i][x] != '.' {
                all = false;
            }
        }
        if all {
            expand_indexes.1.push(x);
        }
    }

    let mut pairs: Vec<(Coords, Coords)> = Vec::new();
    for (i, coord1) in gal.iter().enumerate() {
        for coord2 in gal.iter().skip(i + 1) {
            pairs.push((*coord1, *coord2))
        }
    }

    let mut part_2_total = 0;
    let mut part_1_total = 0;
    for pair in pairs {
        let dist = heuristic_manhattan(&pair.0, &pair.1);
        let mut gaps = 0;
        let mut y = vec![pair.0 .0, pair.1 .0];
        let mut x = vec![pair.0 .1, pair.1 .1];
        y.sort();
        x.sort();

        for row in expand_indexes.0.iter() {
            if row > &y[0] && row < &y[1] {
                gaps += 1;
            }
        }

        for col in expand_indexes.1.iter() {
            if col > &x[0] && col < &x[1] {
                gaps += 1;
            }
        }

        part_1_total += dist + gaps;
        part_2_total += dist + gaps * 999_999
    }

    println!("Part 1 - {}", part_1_total);
    println!("Part 2 - {}", part_2_total);
}
