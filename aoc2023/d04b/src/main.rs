use std::collections::HashSet;

fn main() {
    let puzzle_input = include_str!("../input.txt")
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut card_copies: Vec<usize> = vec![1; puzzle_input.len()];

    let card_total = puzzle_input
        .iter()
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
        .enumerate()
        .map(|(idx, (winners, mine))| {
            let intersect = winners.intersection(&mine);

            let score = intersect.count();

            let mut x = 0;
            while x < card_copies[idx] {
                for i in 1..score + 1 {
                    let val = card_copies[idx + i];
                    card_copies[idx + i] = val + 1;
                }
                x = x + 1;
            }
        });

    for _ in card_total {}
    println!("Final Total: {}", card_copies.iter().sum::<usize>());
}
