fn is_symbol(c: char) -> bool {
    match c {
        '0'..='9' | '.' => false,
        _ => true,
    }
}

fn look_for_symbols(src: &Vec<&str>, idx: (usize, usize)) -> bool {
    let (x, y) = idx;
    let width = src[0].len();
    let height = src.len();

    if x >= width && y >= height {
        return false;
    }

    if x + 1 < width && is_symbol(src[y].chars().nth(x + 1).unwrap()) {
        return true;
    }

    if y + 1 < height && is_symbol(src[y + 1].chars().nth(x).unwrap()) {
        return true;
    }

    if x > 0 && is_symbol(src[y].chars().nth(x - 1).unwrap()) {
        return true;
    }

    if y > 0 && is_symbol(src[y - 1].chars().nth(x).unwrap()) {
        return true;
    }

    if x + 1 < width && y + 1 < height && is_symbol(src[y + 1].chars().nth(x + 1).unwrap()) {
        return true;
    }

    if x > 0 && y > 0 && is_symbol(src[y - 1].chars().nth(x - 1).unwrap()) {
        return true;
    }

    if x > 0 && y + 1 < height && is_symbol(src[y + 1].chars().nth(x - 1).unwrap()) {
        return true;
    }

    if x + 1 < width && y > 0 && is_symbol(src[y - 1].chars().nth(x + 1).unwrap()) {
        return true;
    }

    false
}

fn main() {
    let puzzle_input = include_str!("../input.txt");

    let mut running_sum = 0;

    let split_input = puzzle_input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    for (y, line) in split_input.iter().enumerate() {
        let mut num_str = String::from("");
        let mut is_valid_num = false;
        for (x, chr) in line.chars().enumerate() {
            if chr.is_digit(10) {
                num_str += &chr.to_string();

                if look_for_symbols(&split_input, (x, y)) {
                    is_valid_num = true;
                }
            }
            if !chr.is_digit(10) || x == line.len() - 1 {
                if num_str.len() > 0 {
                    if is_valid_num {
                        running_sum += num_str.parse::<i32>().unwrap();
                    }

                    num_str = String::from("");
                    is_valid_num = false;
                }
            }
        }
    }

    println!("Final Total: {}", running_sum);
}
