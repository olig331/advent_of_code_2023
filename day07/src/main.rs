use std::{collections::HashMap, fs};

#[allow(non_snake_case, non_camel_case_types)]
enum Hands {
    FIVE_KIND,
    FOUR_KIND,
    FULL_HOUSE,
    THREE_KIND,
    TWO_PAIR,
    ONE_PAIR,
    HIGH_C,
}

trait HandToString {
    fn key(&self) -> String;
}
impl HandToString for Hands {
    fn key(&self) -> String {
        match self {
            Hands::FIVE_KIND => String::from("FIVE_KIND"),
            Hands::FOUR_KIND => String::from("FOUR_KIND"),
            Hands::FULL_HOUSE => String::from("FULL_HOUSE"),
            Hands::THREE_KIND => String::from("THREE_KIND"),
            Hands::TWO_PAIR => String::from("TWO_PAIR"),
            Hands::ONE_PAIR => String::from("ONE_PAIR"),
            Hands::HIGH_C => String::from("HIGH_C"),
        }
    }
}

fn append_hand(values: &mut HashMap<String, Vec<(String, u32)>>, h: (&str, &str), key: &str) {
    let val = values.get(key).unwrap();
    let mut new_arr = val.clone();
    new_arr.push((h.0.to_string(), h.1.parse().unwrap()));
    values.insert(key.to_string(), new_arr);
}

fn main() {
    let binding = fs::read_to_string("./input.txt").expect("");
    let input: Vec<(&str, &str)> = binding
        .lines()
        .map(|l| {
            let item: Vec<&str> = l.trim().split_whitespace().collect();
            (item[0], item[1])
        })
        .collect();

    let mut values = gen_values();

    for h in input {
        let hand = eval_hand(h.0);
        let card_values: Vec<u32> = hand.values().cloned().collect();

        if card_values.contains(&5) {
            append_hand(&mut values, h, Hands::FIVE_KIND.key().as_str());
        } else if card_values.contains(&4) {
            append_hand(&mut values, h, Hands::FOUR_KIND.key().as_str());
        } else if card_values.contains(&3) && card_values.contains(&2) {
            append_hand(&mut values, h, Hands::FULL_HOUSE.key().as_str());
        } else if card_values.contains(&3) {
            append_hand(&mut values, h, Hands::THREE_KIND.key().as_str());
        } else if card_values.contains(&2) && hand.len() == 3 {
            append_hand(&mut values, h, Hands::TWO_PAIR.key().as_str());
        } else if card_values.contains(&2) && hand.len() != 3 {
            append_hand(&mut values, h, Hands::ONE_PAIR.key().as_str());
        } else {
            append_hand(&mut values, h, Hands::HIGH_C.key().as_str());
        }
    }

    for (_, v) in &mut values.iter_mut() {
        v.sort_by(|a, b| sort_result(&a.0, &b.0));
    }

    let result: Vec<(String, u32)> = [
        values.get(&Hands::HIGH_C.key()).unwrap(),
        values.get(&Hands::ONE_PAIR.key()).unwrap(),
        values.get(&Hands::TWO_PAIR.key()).unwrap(),
        values.get(&Hands::THREE_KIND.key()).unwrap(),
        values.get(&Hands::FULL_HOUSE.key()).unwrap(),
        values.get(&Hands::FOUR_KIND.key()).unwrap(),
        values.get(&Hands::FIVE_KIND.key()).unwrap(),
    ]
    .iter()
    .flat_map(|v| v.iter().cloned())
    .collect();

    let mut num: u64 = 0;
    for i in 0..result.len() {
        num += result[i].1 as u64 * (i + 1) as u64
    }

    println!("{}", num)
}

fn sort_result(a: &str, b: &str) -> std::cmp::Ordering {
    let a_vals: Vec<usize> = a.chars().map(|c| get_num(c)).collect();
    let b_vals: Vec<usize> = b.chars().map(|c| get_num(c)).collect();

    for (a_val, b_val) in a_vals.iter().zip(b_vals.iter()) {
        match a_val.cmp(b_val) {
            std::cmp::Ordering::Equal => continue,
            ordering => return ordering,
        }
    }

    std::cmp::Ordering::Equal
}

fn get_num(c: char) -> usize {
    match c {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as usize,
    }
}

fn eval_hand(hand: &str) -> HashMap<char, u32> {
    let mut hc = HashMap::new();

    for c in hand.chars() {
        *hc.entry(c).or_insert(0) += 1;
    }

    // part 2 code
    let (mut key, mut val) = ('J', 0);

    if hc.contains_key(&'J') {
        for (k, v) in &hc {
            if k != &'J' {
                if v > &val {
                    val = *v;
                    key = *k;
                } else if v == &val {
                    if get_num(*k) > get_num(key) {
                        key = *k;
                        val = *v;
                    }
                }
            }
        }
    }

    if let Some(num) = hc.get(&'J') {
        if num != &5 {
            hc.insert(key, val + num);
            hc.remove(&'J');
        }
    }

    hc
}

fn gen_values() -> HashMap<String, Vec<(String, u32)>> {
    let mut v: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    v.insert(Hands::FIVE_KIND.key(), Vec::new());
    v.insert(Hands::FOUR_KIND.key(), Vec::new());
    v.insert(Hands::FULL_HOUSE.key(), Vec::new());
    v.insert(Hands::THREE_KIND.key(), Vec::new());
    v.insert(Hands::TWO_PAIR.key(), Vec::new());
    v.insert(Hands::ONE_PAIR.key(), Vec::new());
    v.insert(Hands::HIGH_C.key(), Vec::new());
    v
}
