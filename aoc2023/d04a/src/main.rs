use std::collections::HashSet;

fn main() {
    let puzzle_input = include_str!("../input.txt")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|l| l.split(':').collect::<Vec<_>>())
        .map(|l| l[1].trim())
        .map(|l| l.split('|').map(|s| s.trim()).collect::<Vec<_>>())
        .map(|l| {
            let winning_nums = l[0]
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<HashSet<_>>();

            let my_nums = l[1]
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<HashSet<_>>();

            (winning_nums, my_nums)
        })
        .filter_map(|(winners, mine)| {
            let intersect = winners.intersection(&mine);

            match intersect.count() {
                0 => None,
                1 => Some(1),
                n => Some((0..n - 1).fold(1, |acc, _| acc * 2)),
            }
        })
        .sum::<i32>();

    println!("Final Total: {}", puzzle_input);
}
