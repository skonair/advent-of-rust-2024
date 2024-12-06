use crate::res::res::get_data;

pub mod res;


fn main() {
    // let data = get_example_data();
    let data = get_data();

    let mut lefts = data.lines().map(|line| line.split_whitespace().nth(0).unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    lefts.sort();

    let mut rights = data.lines().map(|line| line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    rights.sort();

    let zip_iter = lefts.iter().zip(rights.iter());

    let mut result = 0;

    for (_i, (l, r)) in zip_iter.enumerate() {
        let d = l - r;
        result = result + d.abs();
    }

    // println!("Left : {}", lefts.iter().map(|i| i.to_string()).collect::<String>());
    // println!("Right: {}", rights.iter().map(|i| i.to_string()).collect::<String>());

    println!("Part 1 is {}", result); // 2378066

    let left_counts = lefts.iter().map(|l| rights.iter().filter(|&n| *n == *l).count()).collect::<Vec<usize>>();

    let zip_iter2 = lefts.iter().zip(left_counts.iter());

    let mut result2 = 0;

    for (_i, (l, lc)) in zip_iter2.enumerate() {
        let d = l * (*lc as i32);
        result2 = result2 + d;
    }

    println!("Part 2 is {}", result2); // 18934359
}
