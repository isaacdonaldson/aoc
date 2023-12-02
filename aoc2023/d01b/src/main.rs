use std::sync::OnceLock;

const INPUT_STR: &str = include_str!("../input.txt");

static WORD_MAP: OnceLock<Vec<(&str, &str)>> = OnceLock::new();

fn set_word_map() {
    WORD_MAP
        .set(vec![
            // Compound numbers
            ("oneight", "18"),
            ("threeight", "38"),
            ("fiveight", "58"),
            ("sevenine", "79"),
            ("nineight", "98"),
            ("twone", "21"),
            ("eightwo", "82"),
            ("eighthree", "83"),
            // Single numbers
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ])
        .unwrap();
}

fn main() {
    set_word_map();

    let mut lines = String::from(INPUT_STR);

    for (key, val) in WORD_MAP.get().unwrap() {
        lines = lines.replace(key, val)
    }

    let total_sum = lines
        .split('\n')
        // only keep the digits
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>())
        .filter(|l| !l.is_empty())
        // only get first and last
        .map(|chrs| (chrs[0], chrs[chrs.len() - 1]))
        // convert both to u32
        .map(|p| (p.0.to_digit(10).unwrap(), p.1.to_digit(10).unwrap()))
        // sum them up
        .map(|p| (p.0 * 10) + p.1)
        // reduce on the sums
        .fold(0, |acc, e| acc + e);

    println!("{}", total_sum);
}
