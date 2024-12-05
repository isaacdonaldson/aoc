use std::collections::HashMap;

const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let rules_n_nums = INPUT_STR.split("\n\n").collect::<Vec<&str>>();
    let [rules_str, nums_str] = rules_n_nums.as_slice() else {
        panic!("Invalid input");
    };

    let rules_vec = rules_str
        .split("\n")
        .map(|r| r.trim())
        .filter(|r| !r.is_empty())
        .map(|r| r.split("|").collect::<Vec<&str>>())
        .map(|r| {
            let [before, after] = r.as_slice() else {
                panic!("Invalid rule");
            };

            (
                before.parse::<i64>().unwrap(),
                after.parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(i64, i64)>>();

    let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();

    for (before, after) in rules_vec {
        let mut vec = rules.get(&before).unwrap_or(&vec![]).clone();
        vec.push(after);
        rules.insert(before, vec.clone());
    }

    let nums = nums_str
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut valid_mid_nums: Vec<i64> = vec![];
    for set in nums {
        let mut encountered: Vec<i64> = vec![];
        let mid_idx = set.len() / 2;
        let mut mid_num = 0;
        let mut valid = true;

        for (idx, n) in set.iter().enumerate() {
            if idx == mid_idx {
                mid_num = *n;
            }

            if let Some(vec) = rules.get(n) {
                for num in vec {
                    if encountered.contains(num) {
                        valid = false;
                        break;
                    }
                }
            }

            if !valid {
                break;
            }

            encountered.push(*n);
        }

        if valid {
            valid_mid_nums.push(mid_num);
        }
    }

    println!("Total: {}", valid_mid_nums.iter().sum::<i64>());
}
