use crate::res::res::get_data;
use std::collections::HashMap;

pub mod res;

fn line_to_tuples(y: i32, line: &str) -> Vec<((i32,i32), char)> {
    return line.char_indices().map(|(x, c)| ((x as i32, y), c)).collect();
}

fn load_grid(data: String) -> HashMap<(i32,i32), char> {

    let lines: HashMap<(i32,i32), char> = data.lines().enumerate()
    .flat_map(|(y, line)| line_to_tuples(y as i32, line))
    .collect();

    // for (key, value) in &lines {
    //     println!("{},{}: {}", key.0, key.1, value);
    // }
    return lines;
}

fn find_words(grid: &HashMap<(i32,i32), char>, pos: (i32, i32), rest: &str) -> i32 {
    let word = String::from(rest);
    let mut chars = word.chars();
    let c = match chars.next() {
        Some(cc) => cc,
        None => '!',
    };

    let is_match = match grid.get(&pos) {
        Some(gc) => *gc == c,
        None => false,
    };


    let x = pos.0;
    let y = pos.1;

    if is_match {
        
        if word.len() == 1 {
            println!("match found on: ({},{}) word: {}", x, y, word);
            1 
        } else {
            println!("find_words: ({},{}) remaining: {}", x, y, word);
            find_words(grid, (x - 1, y - 1), chars.clone().as_str()) +
            find_words(grid, (x, y - 1), chars.clone().as_str()) +
            find_words(grid, (x + 1, y - 1), chars.clone().as_str()) +
            find_words(grid, (x - 1, y), chars.clone().as_str()) +
            find_words(grid, (x + 1, y), chars.clone().as_str()) +
            find_words(grid, (x - 1, y + 1), chars.clone().as_str()) +
            find_words(grid, (x, y + 1), chars.clone().as_str()) +
            find_words(grid, (x + 1, y + 1), chars.clone().as_str())
        }
    } else {
        0
    }
}

fn check_xmas(c1: &char, c2: &char, c3: &char, c4: &char) -> i32 {
    if *c1 == 'X' && *c2 == 'M' && *c3 == 'A' && *c4 == 'S' {
        return 1
    } else {
        return 0
    }
}

fn horizontal_count(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x,y)).unwrap_or(&'!');
    let c2 = grid.get(&(x + 1,y)).unwrap_or(&'!');
    let c3 = grid.get(&(x + 2,y)).unwrap_or(&'!');
    let c4 = grid.get(&(x + 3,y)).unwrap_or(&'!');

    return check_xmas(c1, c2, c3, c4);
}

fn horizontal_backward_count(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x,y)).unwrap_or(&'!');
    let c2 = grid.get(&(x - 1,y)).unwrap_or(&'!');
    let c3 = grid.get(&(x - 2,y)).unwrap_or(&'!');
    let c4 = grid.get(&(x - 3,y)).unwrap_or(&'!');

    return check_xmas(c1, c2, c3, c4);
}

fn vertical_count(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x, y)).unwrap_or(&'!');
    let c2 = grid.get(&(x, y + 1)).unwrap_or(&'!');
    let c3 = grid.get(&(x, y + 2)).unwrap_or(&'!');
    let c4 = grid.get(&(x, y + 3)).unwrap_or(&'!');

    return check_xmas(c1, c2, c3, c4);
}

fn vertical_backward_count(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x, y)).unwrap_or(&'!');
    let c2 = grid.get(&(x, y - 1)).unwrap_or(&'!');
    let c3 = grid.get(&(x, y - 2)).unwrap_or(&'!');
    let c4 = grid.get(&(x, y - 3)).unwrap_or(&'!');

    return check_xmas(c1, c2, c3, c4);
}


fn diagonal_count1(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x, y)).unwrap_or(&'!');
    let c2 = grid.get(&(x + 1, y + 1)).unwrap_or(&'!');
    let c3 = grid.get(&(x + 2, y + 2)).unwrap_or(&'!');
    let c4 = grid.get(&(x + 3, y + 3)).unwrap_or(&'!');

    return check_xmas(c1, c2, c3, c4);
}

fn diagonal_backward_count1(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x, y)).unwrap_or(&'!');
    let c2 = grid.get(&(x - 1, y - 1)).unwrap_or(&'!');
    let c3 = grid.get(&(x - 2, y - 2)).unwrap_or(&'!');
    let c4 = grid.get(&(x - 3, y - 3)).unwrap_or(&'!');

    return check_xmas(c1, c2, c3, c4);
}

fn diagonal_count2(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x, y)).unwrap_or(&'!');
    let c2 = grid.get(&(x - 1, y + 1)).unwrap_or(&'!');
    let c3 = grid.get(&(x - 2, y + 2)).unwrap_or(&'!');
    let c4 = grid.get(&(x - 3, y + 3)).unwrap_or(&'!');

    return check_xmas(c1, c2, c3, c4);
}

fn diagonal_backward_count2(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x, y)).unwrap_or(&'!');
    let c2 = grid.get(&(x + 1, y - 1)).unwrap_or(&'!');
    let c3 = grid.get(&(x + 2, y - 2)).unwrap_or(&'!');
    let c4 = grid.get(&(x + 3, y - 3)).unwrap_or(&'!');

    return check_xmas(c1, c2, c3, c4);
}


fn check_mas(c1: &char, c2: &char, c3: &char) -> i32 {
    if *c1 == 'M' && *c2 == 'A' && *c3 == 'S' {
        return 1
    } else {
        return 0
    }
}

fn mas_count(grid: &HashMap<(i32,i32), char>, pos: (i32, i32)) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let c1 = grid.get(&(x - 1, y - 1)).unwrap_or(&'!');
    let c2 = grid.get(&(x + 1, y - 1)).unwrap_or(&'!');
    let c3 = grid.get(&(x, y)).unwrap_or(&'!');
    let c4 = grid.get(&(x - 1, y + 1)).unwrap_or(&'!');
    let c5 = grid.get(&(x +1, y + 1)).unwrap_or(&'!');

    if (*c3 == 'A') {
        if (*c1 == 'M' && *c5 == 'S') || (*c1 == 'S' && *c5 == 'M') {
            if (*c2 == 'M' && *c4 == 'S') || (*c2 == 'S' && *c4 == 'M') {
                return 1;
            }
        }
    } 
    return 0;
}

fn main() {
    let data = get_data();
    let grid = load_grid(data);

    let mut total = 0;
    for y in 0..140 {
        for x in 0..140 {
            let word_count = horizontal_count(&grid, (x, y)) +
              horizontal_backward_count(&grid, (x, y)) +
              vertical_count(&grid, (x, y)) +
              vertical_backward_count(&grid, (x, y)) +
              diagonal_count1(&grid, (x, y)) +
              diagonal_backward_count1(&grid, (x, y)) +
              diagonal_count2(&grid, (x, y)) +
              diagonal_backward_count2(&grid, (x, y));
            total = total + word_count;
        }
    }
    println!("Total count is {}", total); // 2454 

    let mut total2 = 0;
    for y in 0..140 {
        for x in 0..140 {
            let word_count = mas_count(&grid, (x, y));
            println!("Workd count2 on ({},{}) is {}", x, y, word_count);
            total2 = total2 + word_count;
        }
    }
    println!("Total2 count is {}", total2); // 1858
}
