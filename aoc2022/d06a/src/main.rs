use std::collections::HashSet;

fn main() {
    let puzzle_input = include_bytes!("../input.txt");

    let mut idx = 0;

    let mut word_set: HashSet<&u8> = HashSet::new();

    for (i, word) in puzzle_input.windows(4).enumerate() {
        (0..4).for_each(|n| {
            word_set.insert(&word[n]);
        });

        if word_set.len() == 4 {
            idx = i;
            break;
        }

        word_set.clear();
    }

    println!("idx: {}", idx + 4);
}
