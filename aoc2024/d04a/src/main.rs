const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let letter_matrix = INPUT_STR
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .filter(|l| !l.is_empty())
        .collect::<Vec<Vec<char>>>();

    let row_max = letter_matrix.len() - 1;
    let col_max = letter_matrix[0].len() - 1;

    let mut count = 0;

    for (idx, line) in letter_matrix.iter().enumerate() {
        for (jdx, letter) in line.iter().enumerate() {
            let idx = idx;
            let jdx = jdx;

            if letter != &'X' {
                continue;
            }

            let Some(directions) = calculate_directions(idx, jdx, row_max, col_max) else {
                continue;
            };

            let matches = directions
                .iter()
                .map(|d| {
                    d.iter()
                        .map(|(r, c)| letter_matrix[*r][*c])
                        .collect::<Vec<char>>()
                })
                .filter(|l| check_words(&l))
                .count();

            count += matches;
        }
    }

    println!("Count: {}", count);
}

fn check_words(letters: &[char]) -> bool {
    match letters {
        ['X', 'M', 'A', 'S'] => true,
        _ => false,
    }
}

fn calculate_directions(
    row: usize,
    col: usize,
    row_max: usize,
    col_max: usize,
) -> Option<Vec<[(usize, usize); 4]>> {
    let linear = linear_directions(row, col, row_max, col_max);
    let diagonal = diagonal_directions(row, col, row_max, col_max);

    match (linear, diagonal) {
        (Some(l), Some(d)) => Some(l.into_iter().chain(d.into_iter()).collect()),
        (Some(l), None) => Some(l),
        (None, Some(d)) => Some(d),
        (None, None) => None,
    }
}

fn linear_directions(
    row: usize,
    col: usize,
    row_max: usize,
    col_max: usize,
) -> Option<Vec<[(usize, usize); 4]>> {
    let has_room_forward = col + 3 <= col_max;
    let has_room_backward = col >= 3;
    let has_room_down = row + 3 <= row_max;
    let has_room_up = row >= 3;

    if !has_room_forward && !has_room_backward && !has_room_up && !has_room_down {
        return None;
    }

    let straight_forward = |row, col| [(row, col), (row, col + 1), (row, col + 2), (row, col + 3)];
    let straight_backward = |row, col| [(row, col), (row, col - 1), (row, col - 2), (row, col - 3)];
    let straight_up = |row, col| [(row, col), (row - 1, col), (row - 2, col), (row - 3, col)];
    let straight_down = |row, col| [(row, col), (row + 1, col), (row + 2, col), (row + 3, col)];

    let mut directions = vec![];

    // Checks go clockwise
    if has_room_up {
        directions.push(straight_up(row, col))
    }
    if has_room_forward {
        directions.push(straight_forward(row, col))
    }
    if has_room_down {
        directions.push(straight_down(row, col))
    }
    if has_room_backward {
        directions.push(straight_backward(row, col))
    }
    if directions.is_empty() {
        None
    } else {
        Some(directions)
    }
}

fn diagonal_directions(
    row: usize,
    col: usize,
    row_max: usize,
    col_max: usize,
) -> Option<Vec<[(usize, usize); 4]>> {
    let has_room_forward = col + 3 <= col_max;
    let has_room_backward = col >= 3;
    let has_room_down = row + 3 <= row_max;
    let has_room_up = row >= 3;

    let has_room_up_right = has_room_up && has_room_forward;
    let has_room_up_left = has_room_up && has_room_backward;
    let has_room_down_right = has_room_down && has_room_forward;
    let has_room_down_left = has_room_down && has_room_backward;

    if !has_room_up_right && !has_room_up_left && !has_room_down_right && !has_room_down_left {
        return None;
    }

    let diagonal_up_right = |row, col| {
        [
            (row, col),
            (row - 1, col + 1),
            (row - 2, col + 2),
            (row - 3, col + 3),
        ]
    };
    let diagonal_up_left = |row, col| {
        [
            (row, col),
            (row - 1, col - 1),
            (row - 2, col - 2),
            (row - 3, col - 3),
        ]
    };
    let diagonal_down_right = |row, col| {
        [
            (row, col),
            (row + 1, col + 1),
            (row + 2, col + 2),
            (row + 3, col + 3),
        ]
    };
    let diagonal_down_left = |row, col| {
        [
            (row, col),
            (row + 1, col - 1),
            (row + 2, col - 2),
            (row + 3, col - 3),
        ]
    };

    let mut directions = vec![];

    // Checks go clockwise
    if has_room_up_right {
        directions.push(diagonal_up_right(row, col))
    }
    if has_room_down_right {
        directions.push(diagonal_down_right(row, col))
    }
    if has_room_down_left {
        directions.push(diagonal_down_left(row, col))
    }
    if has_room_up_left {
        directions.push(diagonal_up_left(row, col))
    }

    if directions.is_empty() {
        None
    } else {
        Some(directions)
    }
}
