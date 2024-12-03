const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let mut resulting_arrays: Vec<i64> = vec![];
    let mut idx = 0;

    let arrays = INPUT_STR
        .split('\n')
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .filter(|a| a.len() > 1)
        .map(|l| {
            l.iter()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .inspect(|a| {
            resulting_arrays.push(0);
            if a.len() > idx {
                idx = a.len();
            }
        })
        .collect::<Vec<Vec<i64>>>();

    idx -= 1;

    loop {
        for (i, array) in arrays.iter().enumerate() {
            if resulting_arrays[i] > 0 {
                continue;
            }

            if array.len() > idx || idx == 0 {
                let mut removed = array.clone();
                removed.remove(idx);

                if is_array_valid(removed) {
                    resulting_arrays[i] += 1;
                }
            }
        }

        if idx == 0 {
            break;
        } else {
            idx -= 1;
        }
    }

    let sum = resulting_arrays.iter().filter(|n| **n > 0).count();
    println!("{sum}");
}

fn is_array_valid(arr: Vec<i64>) -> bool {
    let mut sorted = arr.clone();
    sorted.sort();
    if arr != sorted && arr != sorted.iter().rev().cloned().collect::<Vec<i64>>() {
        return false;
    }

    for items in sorted.windows(2) {
        let a = items[0];
        let b = items[1];
        if !(b > a && b - a < 4) {
            return false;
        }
    }

    return true;
}
