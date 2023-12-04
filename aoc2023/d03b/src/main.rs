use std::collections::HashMap;

fn is_symbol(c: char) -> bool {
    match c {
        '*' => true,
        _ => false,
    }
}

fn look_for_symbols(src: &Vec<&str>, idx: (usize, usize)) -> Option<(usize, usize)> {
    let (x, y) = idx;
    let width = src[0].len();
    let height = src.len();

    if x >= width && y >= height {
        return None;
    }

    if x + 1 < width && is_symbol(src[y].chars().nth(x + 1).unwrap()) {
        return Some((x + 1, y));
    }

    if y + 1 < height && is_symbol(src[y + 1].chars().nth(x).unwrap()) {
        return Some((x, y + 1));
    }

    if x > 0 && is_symbol(src[y].chars().nth(x - 1).unwrap()) {
        return Some((x - 1, y));
    }

    if y > 0 && is_symbol(src[y - 1].chars().nth(x).unwrap()) {
        return Some((x, y - 1));
    }

    if x + 1 < width && y + 1 < height && is_symbol(src[y + 1].chars().nth(x + 1).unwrap()) {
        return Some((x + 1, y + 1));
    }

    if x > 0 && y > 0 && is_symbol(src[y - 1].chars().nth(x - 1).unwrap()) {
        return Some((x - 1, y - 1));
    }

    if x > 0 && y + 1 < height && is_symbol(src[y + 1].chars().nth(x - 1).unwrap()) {
        return Some((x - 1, y + 1));
    }

    if x + 1 < width && y > 0 && is_symbol(src[y - 1].chars().nth(x + 1).unwrap()) {
        return Some((x + 1, y - 1));
    }

    None
}

fn main() {
    let puzzle_input = include_str!("../input.txt");

    // Vec of (gear_num, '*' symbol points)
    let mut found_gears: Vec<(i32, (usize, usize))> = vec![];

    let split_input = puzzle_input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    for (y, line) in split_input.iter().enumerate() {
        let mut num_str = String::from("");
        let mut is_valid_num = false;
        let mut star_point = (0, 0);
        for (x, chr) in line.chars().enumerate() {
            if chr.is_digit(10) {
                num_str += &chr.to_string();

                if let Some(found_point) = look_for_symbols(&split_input, (x, y)) {
                    is_valid_num = true;
                    star_point = found_point;
                }
            }
            if !chr.is_digit(10) || x == line.len() - 1 {
                if num_str.len() > 0 {
                    if is_valid_num {
                        found_gears.push((num_str.parse::<i32>().unwrap(), star_point));
                    }

                    num_str = String::from("");
                    is_valid_num = false;
                    star_point = (0, 0);
                }
            }
        }
    }

    let star_counts: HashMap<(usize, usize), i32> =
        found_gears
            .iter()
            .fold(HashMap::new(), |mut acc, (_, star_point)| {
                let count = acc.get(star_point).unwrap_or(&0);
                println!("{star_point:?} -> {:?}", acc);
                acc.insert(*star_point, count + 1);
                acc
            });

    let total_num = found_gears
        .iter()
        .filter(|(_, star_points)| {
            let count = star_counts.get(star_points).unwrap_or(&0);
            *count > 1
        })
        .fold(HashMap::new(), |mut acc, (gear, star_point)| {
            let ratio = acc.get(star_point).unwrap_or(&1);
            acc.insert(*star_point, gear * *ratio);
            acc
        })
        .iter()
        .fold(0, |acc, (_, val)| acc + val);

    println!("Final Total: {}", total_num);
}
