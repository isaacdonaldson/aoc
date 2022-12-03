use std::collections::HashSet;

const BAG_INPUT: &str = include_str!("../input.txt");

fn main() {
    let lines = BAG_INPUT.split("\n");

    let mut total_priorities = 0;

    for line in lines {
        let mut first: HashSet<u8> = HashSet::new();
        let mut second: HashSet<u8> = HashSet::new();
        for (idx, char) in line.chars().enumerate() {
            if idx < (line.len() / 2) {
                // print!("{char}--");
                first.insert(char as u8);
            } else {
                // print!("||{char}");
                second.insert(char as u8);
            }
        }

        let intersection = first.intersection(&second).collect::<Vec<_>>()[0];

        // add to sum ass ascii value
        total_priorities += if intersection >= &b'a' {
            (intersection - b'a') as i16 + 1
        } else {
            (intersection - b'A') as i16 + 27
        };
    }

    println!("{:?}", total_priorities);
}
