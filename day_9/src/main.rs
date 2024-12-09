
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;


fn get_input() -> Vec<u32> {
    let input = std::fs::read_to_string("input").unwrap();
    input
        .trim()
        .chars()
        .map(|elem| elem.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .unwrap()
}

pub fn p_1()  {
    let memblocks = get_input();
    let mut memory_representation: Vec<String> = memblocks
        .iter()
        .enumerate()
        .flat_map(|(i, val)| {
            let id = if i % 2 == 0 {
                (i / 2).to_string()
            } else {
                String::from(".")
            };
            (0..*val).map(|_| id.clone()).collect::<Vec<String>>()
        })
        .collect();

    let (mut head, mut tail) = (0, memory_representation.len() - 1);

    while head < tail {
        if &memory_representation[head] == "." {
            memory_representation.swap(head, tail);
            while &memory_representation[tail] == "." && tail > head {
                tail -= 1;
            }
        }
        head += 1;
    }

    let checksum = memory_representation
        .iter()
        .enumerate()
        .fold_while(0, |acc, (index, id)| {
            if id == "." {
                Done(acc)
            } else {
                let id_parsed = id.parse::<usize>().unwrap();
                Continue(acc + (index * id_parsed))
            }
        })
        .into_inner();

    println!("P1: {}", checksum);

}

pub fn p_2() {
    let memblocks = get_input();
    let mut max_file_index: usize = 0;

    let mut mem_map: Vec<(usize, usize, bool)> = memblocks
        .into_iter()
        .enumerate()
        .map(|(index, val)| {
            if index % 2 == 0 {
                max_file_index = index / 2;
                (index / 2, val as usize, true)
            } else {
                (0, val as usize, false)
            }
        })
        .collect();

    for i in (1..=max_file_index).rev() {
        let (index_of_movable_chunk, (_, occupation, _)) = mem_map
            .iter()
            .find_position(|(chunk_id, _, is_file)| *is_file && *chunk_id == i)
            .unwrap();
        let mut new_mem_map = vec![];

        let mut chunk_moved = false;

        for (index, (chunk_id, chunk_occ, is_file)) in mem_map.iter().enumerate() {
            if *is_file {
                if *chunk_id != i || !chunk_moved {
                    new_mem_map.push((*chunk_id, *chunk_occ, *is_file));
                } else {
                    let (_, occ, _) = new_mem_map.last_mut().unwrap();
                    *occ += chunk_occ;
                }
            } else if index > index_of_movable_chunk || *chunk_occ < *occupation || chunk_moved {
                new_mem_map.push((*chunk_id, *chunk_occ, *is_file));
            } else {
                chunk_moved = true;
                new_mem_map.push((0, 0, false));
                new_mem_map.push((i, *occupation, true));
                new_mem_map.push((0, chunk_occ - occupation, false));
            }
        }

        mem_map = new_mem_map;
    }

    let (_, total) = mem_map
        .into_iter()
        .fold((0, 0), |(i, sum), (id, occupation, is_file)| {
            if !is_file {
                (i + occupation, sum)
            } else {
                let mut file_sum = 0;
                for j in i..(i + occupation) {
                    file_sum += j * id;
                }
                (i + occupation, sum + file_sum)
            }
        });

    println!(
        "P2: {}",
        total
    );

}

fn main() {
    p_1();
    p_2();
}