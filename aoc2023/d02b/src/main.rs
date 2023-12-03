use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
enum CubeColor {
    Red,
    Blue,
    Green,
}

fn main() {
    let puzzle_input = include_str!("../input.txt");
    let mut puzzle_hash: HashMap<(i64, CubeColor), i64> = HashMap::new();
    let mut init_game = |game_id| {
        puzzle_hash.insert((game_id, CubeColor::Red), 0);
        puzzle_hash.insert((game_id, CubeColor::Green), 0);
        puzzle_hash.insert((game_id, CubeColor::Blue), 0);
    };

    let id_lines: Vec<(i64, Vec<&str>)> = puzzle_input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split(':'))
        .map(|pair| {
            let pair = pair.collect::<Vec<&str>>();
            let game_id = pair[0].split(' ').last().unwrap().parse::<u32>().unwrap();
            init_game(game_id as i64);
            (
                game_id as i64,
                pair.last().unwrap().split(';').collect::<Vec<&str>>(),
            )
        })
        .collect();

    for line in id_lines {
        let (id, set) = line;
        for game in set {
            for hand in game.split(',') {
                let split = hand.trim().split(' ').collect::<Vec<&str>>();
                let num = split[0].parse::<i64>().unwrap();
                let color = split[1];
                match color.trim() {
                    "red" => {
                        let prev = puzzle_hash.get(&(id, CubeColor::Red)).unwrap_or(&0i64);
                        if *prev < num {
                            puzzle_hash.insert((id, CubeColor::Red), num);
                        }
                    }
                    "green" => {
                        let prev = puzzle_hash.get(&(id, CubeColor::Green)).unwrap_or(&0i64);
                        if *prev < num {
                            puzzle_hash.insert((id, CubeColor::Green), num);
                        }
                    }
                    "blue" => {
                        let prev = puzzle_hash.get(&(id, CubeColor::Blue)).unwrap_or(&0i64);
                        if *prev < num {
                            puzzle_hash.insert((id, CubeColor::Blue), num);
                        }
                    }
                    _ => (),
                };
            }
        }
    }

    let mut score_hash: HashMap<i64, i64> = HashMap::new();

    let _ = puzzle_hash
        .iter()
        .map(|(key, val)| {
            let (game_id, _) = key;
            let prev = score_hash.get(game_id).unwrap_or(&1i64);
            score_hash.insert(*game_id, prev * val);
        })
        .fold(0, |acc, _| acc);

    let final_total = score_hash.iter().fold(0, |acc, (_, val)| acc + val);

    println!("Final Total: {}", final_total);
}
