
const INPUT_MOVES : &str = include_str!("../input.txt");

#[repr(u8)]
#[derive(Copy, Clone)]
enum GameScores {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum GameMoves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl GameMoves {
    fn from_opp_str(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("Invalid Oppenent move"),
        }
    }

    fn from_result_str(s: &str) -> GameScores {
        match s {
            "X" => GameScores::Loss,
            "Y" => GameScores::Draw,
            "Z" => GameScores::Win,
            _ => panic!("Invalid Game Result"),
        }
    }

    fn parse_line(s: &str) -> (Self, GameScores) {
        let mut split = s.split_whitespace();
        let opp = Self::from_opp_str(split.next().unwrap());
        let result = Self::from_result_str(split.next().unwrap());
        (opp, result)
    }

    fn calculate_my_move(opp: GameMoves, result: GameScores) -> GameMoves {
        match (opp, result) {
            (GameMoves::Rock, GameScores::Loss) => GameMoves::Scissors,
            (GameMoves::Rock, GameScores::Draw) => GameMoves::Rock,
            (GameMoves::Rock, GameScores::Win) => GameMoves::Paper,
            (GameMoves::Paper, GameScores::Loss) => GameMoves::Rock,
            (GameMoves::Paper, GameScores::Draw) => GameMoves::Paper,
            (GameMoves::Paper, GameScores::Win) => GameMoves::Scissors,
            (GameMoves::Scissors, GameScores::Loss) => GameMoves::Paper,
            (GameMoves::Scissors, GameScores::Draw) => GameMoves::Scissors,
            (GameMoves::Scissors, GameScores::Win) => GameMoves::Rock,
        }
    }

    fn calculate_score(move_played: GameMoves, match_result: GameScores) -> u16 {
        move_played as u16 + match_result as u16
    }

}


fn main() {
    let mut my_score = 0;

    for line in INPUT_MOVES.lines() {
        let (opp, result) = GameMoves::parse_line(line);
        let my_move = GameMoves::calculate_my_move(opp, result);
        my_score += GameMoves::calculate_score(my_move, result);
    }

    println!("My score: {my_score}");

}
