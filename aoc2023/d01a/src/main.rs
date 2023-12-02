const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let total_sum = INPUT_STR
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
