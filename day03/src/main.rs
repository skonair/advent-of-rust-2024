use crate::res::res::get_data;
use regex::Regex;

pub mod res;

fn main() {
    let data = get_data();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let products: Vec<i32> = re.captures_iter(data.as_str()).map(|caps| {
        println!("caps[1] is {}", &caps[1]);
        println!("caps[2] is {}", &caps[2]);
        return &caps[1].parse::<i32>().unwrap() * &caps[2].parse::<i32>().unwrap();
    }
    ).collect();

    let sum: i32 = products.iter().sum();

    println!("Part1 is {}", sum); // 159892596

    let re2 = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let sum2: (i32, bool) = re2.captures_iter(data.as_str()).fold((0i32,true), |val, caps| {
        let newflag = if &caps[0] == "do()" { true } else if &caps[0] == "don't()" { false } else { val.1 };
        let newval = if newflag && *&caps[0].starts_with("mul") {
            let prod = &caps[1].parse::<i32>().unwrap() * &caps[2].parse::<i32>().unwrap();
            prod
        } else { 0 };
        return (val.0 + newval, newflag);
    });

    println!("Part2 is {}", sum2.0); // 92626942

}
