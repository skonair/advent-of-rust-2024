use crate::res::res::get_data;

pub mod res;

fn is_valid_up(line: &Vec<i32>) -> bool {
    let is_valid_up = line.windows(2).all(|w| w[0] < w[1] && (w[0] - w[1]).abs() <= 3);
    return is_valid_up;
}

fn is_valid_down(line: &Vec<i32>) -> bool {
    let is_valid_down = line.windows(2).all(|w| w[0] > w[1] && (w[0] - w[1]).abs() <= 3);
    return is_valid_down;
}

fn is_valid(line: &Vec<i32>) -> bool {
    let is_valid_up = is_valid_up(line);
    let is_valid_down = is_valid_down(line);
    return is_valid_up || is_valid_down;
}

fn is_valid_part2(list: &Vec<i32>) -> bool {
    for (i, _e) in list.iter().enumerate() {
        let mut copied = list.clone();
        copied.remove(i);
        if is_valid (&copied) {
            return true;
        }
    }
    return false;
}

fn main() {
    // let data = get_example_data();
        let data = get_data();

    let lines = data.lines()
      .map(|line| line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
      .collect::<Vec<Vec<i32>>>();

    let mut is_valid_count = 0; 
    let mut is_valid_count_part2 = 0; 

    for line in lines.iter() {
        if is_valid(line) {
            is_valid_count = is_valid_count + 1;
        }

        if is_valid_part2(line) {
            is_valid_count_part2 = is_valid_count_part2 + 1;
        }

        println!("line: {}", line.iter().map(|i| i.to_string()).collect::<String>());
    }

    println!("is_valid_count_part1: {}", is_valid_count); // 246
    println!("is_valid_count_part2: {}", is_valid_count_part2); // 318
}
