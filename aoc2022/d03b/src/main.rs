use std::collections::HashSet;

const BAG_INPUT: &str = include_str!("../input.txt");

fn main() {
    let all_lines: Vec<&str> = BAG_INPUT.split("\n").collect();

    let lines = all_lines.chunks(3);

    let mut total_priorities = 0;

    for group in lines {
        let first: HashSet<u8> = group[0].bytes().collect();
        let second: HashSet<u8> = group[1].bytes().collect();
        let third: HashSet<u8> = group[2].bytes().collect();

        let intersection_set: Vec<u8> = first
            .iter()
            .filter(|&x| second.contains(x) && (&third).contains(x))
            .map(|&x| x)
            .collect();

        let intersection = intersection_set[0];

        // add to sum ass ascii value
        total_priorities += if intersection >= b'a' {
            (intersection - b'a') as i16 + 1
        } else {
            (intersection - b'A') as i16 + 27
        };
    }

    println!("{:?}", total_priorities);
}
