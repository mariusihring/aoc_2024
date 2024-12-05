use std::collections::HashMap;
use std::collections::HashSet;

fn split_data(input: String) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();

    let rules: Vec<(i64, i64)> = parts
        .get(0)
        .unwrap_or(&"")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| {
            let nums: Vec<i64> = s
                .split('|')
                .filter_map(|x| x.trim().parse::<i64>().ok())
                .collect();
            (nums[0], nums[1])
        })
        .collect();

    let updates: Vec<Vec<i64>> = parts
        .get(1)
        .unwrap_or(&"")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.trim()
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn is_valid_update(rules: &Vec<(i64, i64)>, update: &Vec<i64>) -> bool {
    let positions: HashMap<i64, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &val)| (val, i))
        .collect();

    for (a, b) in rules {
        if let (Some(&pos_a), Some(&pos_b)) = (positions.get(a), positions.get(b)) {
            if pos_a > pos_b {
                return false;
            }
        }
    }
    true
}

fn corrected_version(rules: &Vec<(i64, i64)>, update: &Vec<i64>) -> Vec<i64> {
    let mut n = update.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for (a, b) in rules {
            if let (Some(pos_a), Some(pos_b)) =
                (n.iter().position(|x| x == a), n.iter().position(|x| x == b))
            {
                if pos_b < pos_a {
                    let b_val = n.remove(pos_b);
                    let new_a_pos = n.iter().position(|x| x == a).unwrap();
                    n.insert(new_a_pos + 1, b_val);
                    changed = true;
                }
            }
        }
    }
    n
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (rules, updates) = split_data(input);

    let mut part_1 = 0;
    for update in &updates {
        if is_valid_update(&rules, update) {
            let middle = update[update.len() / 2];
            part_1 += middle;
        }
    }
    let mut part_2 = 0;
    for update in &updates {
        if !is_valid_update(&rules, update) {
            let y = corrected_version(&rules, update);
            let middle = y[y.len() / 2];
            part_2 += middle;
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
