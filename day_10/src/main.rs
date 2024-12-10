use std::collections::HashMap;

fn get_map() -> Vec<Vec<i32>> {
    let input = include_str!("../input");
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn trail_heads(map: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut starts = Vec::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == 0 {
                starts.push((row_idx, col_idx));
            }
        }
    }
    starts
}
fn count_paths(
    map: &Vec<Vec<i32>>,
    current: (usize, usize),
    visited: &mut Vec<(usize, usize)>,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let current_height = map[current.0][current.1];

    if current_height == 9 {
        return 1;
    }

    // Create cache key  current position & visited length
    let cache_key = (current.0, current.1, visited.len());
    if let Some(&cached) = cache.get(&cache_key) {
        return cached;
    }

    let max_row = map.len() as isize - 1;
    let max_col = map[0].len() as isize - 1;
    let mut total_paths = 0;

    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let next_row = current.0 as isize + dx;
        let next_col = current.1 as isize + dy;

        if next_row >= 0 && next_row <= max_row && next_col >= 0 && next_col <= max_col {
            let next = (next_row as usize, next_col as usize);
            let next_height = map[next.0][next.1];

            if next_height == current_height + 1 && !visited.contains(&next) {
                visited.push(next);
                total_paths += count_paths(map, next, visited, cache);
                visited.pop();
            }
        }
    }

    cache.insert(cache_key, total_paths);
    total_paths
}

fn p_2() {
    let map = get_map();
    let starts = trail_heads(map.clone());
    let mut total_rating = 0;
    let mut cache = HashMap::new();

    for &start in &starts {
        let mut visited = vec![start];
        let paths = count_paths(&map, start, &mut visited, &mut cache);
        //println!("Trailhead {:?} has rating: {}", start, paths);
        total_rating += paths;
    }

    println!("P2: {}", total_rating);
}
fn walk_trail(map: Vec<Vec<i32>>, heads: Vec<(usize, usize)>) -> usize {
    let mut result = 0;
    let max_row = map.len() as isize - 1;
    let max_col = map[0].len() as isize - 1;

    //is in map
    fn is_valid_position(pos: (isize, isize), max_row: isize, max_col: isize) -> bool {
        pos.0 >= 0 && pos.0 <= max_row && pos.1 >= 0 && pos.1 <= max_col
    }

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn calc(&self) -> (isize, isize) {
            match self {
                Direction::Up => (-1, 0),
                Direction::Down => (1, 0),
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
            }
        }
    }

    for head in heads {
        let mut reachable_tails = 0;
        let mut visited: Vec<(isize, isize)> = vec![(head.0 as isize, head.1 as isize)];
        let mut items_to_check: Vec<(isize, isize)> = vec![(head.0 as isize, head.1 as isize)];
        let mut can_walk = true;

        while can_walk && !items_to_check.is_empty() {
            let mut new_items = Vec::new();

            for &item in &items_to_check {
                let current_height = map[item.0 as usize][item.1 as usize];

                for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                    let offset = direction.calc();
                    let next_pos = (item.0 + offset.0, item.1 + offset.1);

                    // valid and not visited
                    if is_valid_position(next_pos, max_row, max_col) && !visited.contains(&next_pos) {
                        let next_height = map[next_pos.0 as usize][next_pos.1 as usize];

                        if next_height == current_height + 1 {
                            if next_height == 9 {
                                reachable_tails += 1;
                            } else {
                                new_items.push(next_pos);
                            }
                            visited.push(next_pos);
                        }
                    }
                }
            }

            items_to_check = new_items;

            if items_to_check.is_empty() {
                can_walk = false;
                result += reachable_tails;
                println!("head {:?} reachable tails: {:?}", head, reachable_tails);
            }
        }
    }
    result
}

fn p_1() {
    let map = get_map();
    let starts = trail_heads(map.clone());
    let res = walk_trail(map, starts);
    println!("P1: {}", res);
}


fn main() {
    p_1();
    p_2();
}
