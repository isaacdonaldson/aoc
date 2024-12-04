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

            if letter != &'A' {
                continue;
            }

            let Some(directions) = diagonal_directions(idx, jdx, row_max, col_max) else {
                continue;
            };

            let word_cross: Vec<Vec<char>> = directions
                .iter()
                .map(|d| {
                    d.iter()
                        .map(|(r, c)| letter_matrix[*r][*c])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();

            if check_words(&word_cross) {
                count += 1;
            }
        }
    }

    println!("Count: {}", count);
}

fn check_words(directions: &Vec<Vec<char>>) -> bool {
    let [slant_right, slant_left] = directions.as_slice() else {
        return false;
    };

    match (slant_right.as_slice(), slant_left.as_slice()) {
        (['S', 'A', 'M'], ['S', 'A', 'M']) => true,
        (['M', 'A', 'S'], ['M', 'A', 'S']) => true,
        (['M', 'A', 'S'], ['S', 'A', 'M']) => true,
        (['S', 'A', 'M'], ['M', 'A', 'S']) => true,
        _ => false,
    }
}

fn diagonal_directions(
    row: usize,
    col: usize,
    row_max: usize,
    col_max: usize,
) -> Option<Vec<[(usize, usize); 3]>> {
    let has_room_forward = col + 1 <= col_max;
    let has_room_backward = col >= 1;
    let has_room_down = row + 1 <= row_max;
    let has_room_up = row >= 1;

    let has_room_up_right = has_room_up && has_room_forward;
    let has_room_up_left = has_room_up && has_room_backward;
    let has_room_down_right = has_room_down && has_room_forward;
    let has_room_down_left = has_room_down && has_room_backward;

    let has_slant_right = has_room_up_right && has_room_down_left;
    let has_slant_left = has_room_up_left && has_room_down_right;

    if !has_slant_left && !has_slant_right {
        return None;
    }

    let diagonal_slant_right = |row, col| [(row + 1, col - 1), (row, col), (row - 1, col + 1)];
    let diagonal_slant_left = |row, col| [(row - 1, col - 1), (row, col), (row + 1, col + 1)];

    Some(vec![
        diagonal_slant_right(row, col),
        diagonal_slant_left(row, col),
    ])
}
