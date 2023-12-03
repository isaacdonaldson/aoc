use std::collections::HashSet;

const RED_CUBES: i64 = 12;
const GREEN_CUBES: i64 = 13;
const BLUE_CUBES: i64 = 14;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    let id_lines: Vec<(i64, Vec<&str>)> = puzzle_input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split(':'))
        .map(|pair| {
            let pair = pair.collect::<Vec<&str>>();
            let game_id = pair[0].split(' ').last().unwrap().parse::<u32>().unwrap();
            (
                game_id as i64,
                pair.last().unwrap().split(';').collect::<Vec<&str>>(),
            )
        })
        .collect();

    let mut possibles: HashSet<i64> = HashSet::new();
    let mut impossibles: HashSet<i64> = HashSet::new();

    for line in id_lines {
        let (id, set) = line;
        for game in set {
            for hand in game.split(',') {
                let split = hand.trim().split(' ').collect::<Vec<&str>>();
                let num = split[0].parse::<i64>().unwrap();
                let color = split[1];
                match color.trim() {
                    "red" => {
                        if num > RED_CUBES {
                            impossibles.insert(id);
                        }
                        possibles.insert(id)
                    }
                    "green" => {
                        if num > GREEN_CUBES {
                            impossibles.insert(id);
                        }
                        possibles.insert(id)
                    }
                    "blue" => {
                        if num > BLUE_CUBES {
                            impossibles.insert(id);
                        }
                        possibles.insert(id)
                    }
                    _ => false,
                };
            }
        }
    }

    let resulting_game_id_total = possibles
        .iter()
        .filter(|id| !impossibles.contains(id))
        .fold(0, |acc, id| acc + id);

    println!("Final Total: {resulting_game_id_total}");
}
