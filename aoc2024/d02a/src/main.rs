const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let arrays = INPUT_STR
        .split('\n')
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .filter(|a| a.len() > 1)
        .map(|l| {
            l.iter()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let filtered = arrays
        .iter()
        .filter(|l| {
            let mut sorted = (*l).clone();
            sorted.sort();
            **l == sorted || **l == sorted.iter().rev().cloned().collect::<Vec<i64>>()
        })
        .map(|l| {
            let mut s = l.clone();
            s.sort();
            s
        })
        .collect::<Vec<Vec<i64>>>();

    let mut total_unsafe = 0;

    for array in filtered.clone() {
        for items in array.windows(2) {
            let a = items[0];
            let b = items[1];
            if !(b > a && b - a < 4) {
                total_unsafe += 1;
                break;
            }
        }
    }

    let total_safe = filtered.len() - total_unsafe;

    println!("{total_safe}");
}
