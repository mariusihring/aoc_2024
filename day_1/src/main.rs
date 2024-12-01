fn p_1() {
    let mut  list_1: Vec<i64> = Vec::new();
    let mut  list_2: Vec<i64> = Vec::new();
    let x  = std::fs::read_to_string("/Users/mariusihring/Developer/aoc_2024/day_1/src/input").unwrap().lines().for_each(|line| {
        let nums = line.split(' ').collect::<Vec<&str>>();

        list_1.push(nums[0].parse::<i64>().unwrap());
        list_2.push(nums[3].parse::<i64>().unwrap());
    });


    list_1.sort();
    list_2.sort();

    let mut result: i64 = 0;
    for (index, num) in list_1.iter().enumerate() {
        let x = num - list_2.get(index).unwrap();
        result += x.abs();
    }

    println!("Result: {}", result);
}

fn p_2() {
    let mut  list_1: Vec<i64> = Vec::new();
    let mut  list_2: Vec<i64> = Vec::new();
    let x  = std::fs::read_to_string("/Users/mariusihring/Developer/aoc_2024/day_1/src/input").unwrap().lines().for_each(|line| {
        let nums = line.split(' ').collect::<Vec<&str>>();

        list_1.push(nums[0].parse::<i64>().unwrap());
        list_2.push(nums[3].parse::<i64>().unwrap());
    });



    let mut result: i64 = 0;
    for x in list_1.iter() {
        let multiplier = list_2.iter().filter(|&y| x == y).count() as i64;

        result += x * multiplier;
    }
    println!("Result: {}", result);
}

fn main() {
    p_1();
    p_2();
}
