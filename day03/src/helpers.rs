use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone)]
pub struct Value {
    pub start: usize,
    pub end: usize,
    pub val: u64,
}

pub(crate) fn insert_to_map(map: &mut HashMap<String, Vec<u64>>, key: String, val: u64) {
    match map.entry(key.clone()) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().push(val);
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![val]);
        }
    }
}

pub(crate) fn has_symbol(input: &str) -> bool {
    input
        .replace(".", "")
        .chars()
        .any(|c| c.is_ascii_punctuation())
}

pub(crate) fn gen_indices(line: &str) -> Vec<Value> {
    let mut items = Vec::new();
    let mut start_index = None;

    for (index, c) in line.char_indices() {
        if c.is_digit(10) {
            if start_index.is_none() {
                start_index = Some(index);
            }
        } else if let Some(start) = start_index {
            let end_index = index - 1;
            let number = &line[start..=end_index];
            items.push(Value {
                val: number.parse().unwrap(),
                start,
                end: end_index + 1,
            });
            start_index = None;
        }
    }

    if let Some(start) = start_index {
        let end_index = line.len() - 1;
        let number = &line[start..=end_index];
        items.push(Value {
            val: number.parse().unwrap(),
            start,
            end: end_index + 1,
        });
    }

    items
}

pub(crate) fn get_slices(
    grid: &[String],
    y: usize,
    x_start_val: usize,
    x_end_val: usize,
    item: &Value,
) -> (String, String, String, String) {
    let up_slice = if y > 0 {
        grid[y - 1][x_start_val..x_end_val].to_string()
    } else {
        String::new()
    };

    let down_slice = if y < grid.len() - 1 {
        grid[y + 1][x_start_val..x_end_val].to_string()
    } else {
        String::new()
    };

    let right_slice = if item.end < grid[y].len() - 1 {
        grid[y][item.end..item.end + 1].to_string()
    } else {
        String::new()
    };

    let left_slice = if item.start > 0 {
        grid[y][item.start - 1..item.start].to_string()
    } else {
        String::new()
    };

    (up_slice, down_slice, right_slice, left_slice)
}
