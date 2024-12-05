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

    let mut proper_rules: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut improper_rules: HashMap<i64, Vec<i64>> = HashMap::new();

    for (before, after) in rules_vec {
        let mut proper_vec = proper_rules.get(&before).unwrap_or(&vec![]).clone();
        let mut improper_vec = improper_rules.get(&after).unwrap_or(&vec![]).clone();
        proper_vec.push(after);
        improper_vec.push(before);

        proper_rules.insert(before, proper_vec.clone());
        improper_rules.insert(after, improper_vec.clone());
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

    let mut mid_nums: Vec<i64> = vec![];

    for set in nums {
        let mut encountered: Vec<i64> = vec![];
        let mid_idx = set.len() / 2;
        let mut valid = true;

        for n in &set {
            if let Some(vec) = proper_rules.get(n) {
                for num in vec {
                    if encountered.contains(num) {
                        let reordered_set = reorder_incorrect_set(&set, &proper_rules);
                        mid_nums.push(reordered_set[mid_idx]);
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
    }

    println!("Total: {}", mid_nums.iter().sum::<i64>());
}

fn reorder_incorrect_set(set: &Vec<i64>, rules: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    let mut reordered_set = set.clone();

    reordered_set.sort_by(|a, b| {
        if let Some(befores) = rules.get(a) {
            if befores.contains(b) {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        } else {
            return std::cmp::Ordering::Greater;
        }
    });

    reordered_set
}
