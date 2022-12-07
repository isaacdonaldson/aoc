use std::collections::HashSet;

fn main() {
    let puzzle_input = include_bytes!("../input.txt");

    let mut idx = 0;

    let mut word_set: HashSet<&u8> = HashSet::new();

    for (i, word) in puzzle_input.windows(14).enumerate() {
        (0..14).for_each(|n| {
            word_set.insert(&word[n]);
        });

        if word_set.len() == 14 {
            idx = i;
            break;
        }

        word_set.clear();
    }

    println!("idx: {}", idx + 14);
}
