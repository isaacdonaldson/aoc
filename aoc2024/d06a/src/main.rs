use std::collections::HashSet;

const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let map = INPUT_STR
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split("").filter(|c| *c != "").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut guard_starting_position: (usize, usize) = (0, 0);
    let mut guard_direction: Direction = Direction::Up;
    let mut obstacles: Vec<(usize, usize)> = vec![];

    for (idx, row) in map.iter().enumerate() {
        for (jdx, cell) in row.iter().enumerate() {
            match *cell {
                "^" => {
                    guard_starting_position = (idx, jdx);
                    guard_direction = Direction::Up;
                }
                ">" => {
                    guard_starting_position = (idx, jdx);
                    guard_direction = Direction::Right;
                }
                "v" => {
                    guard_starting_position = (idx, jdx);
                    guard_direction = Direction::Down;
                }
                "<" => {
                    guard_starting_position = (idx, jdx);
                    guard_direction = Direction::Left;
                }
                "#" => {
                    obstacles.push((idx, jdx));
                }
                _ => {}
            }
        }
    }

    let mut gcp = guard_starting_position;
    let mut visited_sqrs: Vec<(usize, usize)> = vec![];
    while (gcp.0 < map.len()) && (gcp.1 < map[0].len()) {
        let next_pos = match guard_direction {
            Direction::Up => (gcp.0 - 1, gcp.1),
            Direction::Right => (gcp.0, gcp.1 + 1),
            Direction::Down => (gcp.0 + 1, gcp.1),
            Direction::Left => (gcp.0, gcp.1 - 1),
            _ => break,
        };

        if obstacles.contains(&next_pos) {
            guard_direction = guard_direction.next_dir();
        } else {
            gcp = next_pos;
            visited_sqrs.push(gcp);
        }
    }

    let unique_squares = visited_sqrs.iter().collect::<HashSet<_>>();

    println!("visits: {}", unique_squares.len() - 1);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_dir(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
