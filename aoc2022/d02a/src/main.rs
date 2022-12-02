
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

    fn from_my_str(s: &str) -> Self {
        match s {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("Invalid Player move"),
        }
    }

    fn parse_line(s: &str) -> (Self, Self) {
        let mut split = s.split_whitespace();
        let opp = Self::from_opp_str(split.next().unwrap());
        let my = Self::from_my_str(split.next().unwrap());
        (opp, my)
    }

    fn calculate_result_pair(opp: GameMoves, my: GameMoves) -> (GameScores, GameScores) {
        match (opp, my) {
            (GameMoves::Rock, GameMoves::Rock) => (GameScores::Draw, GameScores::Draw),
            (GameMoves::Rock, GameMoves::Paper) => (GameScores::Loss, GameScores::Win),
            (GameMoves::Rock, GameMoves::Scissors) => (GameScores::Win, GameScores::Loss),
            (GameMoves::Paper, GameMoves::Rock) => (GameScores::Win, GameScores::Loss),
            (GameMoves::Paper, GameMoves::Paper) => (GameScores::Draw, GameScores::Draw),
            (GameMoves::Paper, GameMoves::Scissors) => (GameScores::Loss, GameScores::Win),
            (GameMoves::Scissors, GameMoves::Rock) => (GameScores::Loss, GameScores::Win),
            (GameMoves::Scissors, GameMoves::Paper) => (GameScores::Win, GameScores::Loss),
            (GameMoves::Scissors, GameMoves::Scissors) => (GameScores::Draw, GameScores::Draw),
        }
    }

    fn calculate_score(move_played: GameMoves, match_result: GameScores) -> u16 {
        move_played as u16 + match_result as u16
    }

}


fn main() {
    let mut opp_score = 0;
    let mut my_score = 0;

    for line in INPUT_MOVES.lines() {
        let (opp, my) = GameMoves::parse_line(line);
        let (opp_result, my_result) = GameMoves::calculate_result_pair(opp, my);
        opp_score += GameMoves::calculate_score(opp, opp_result);
        my_score += GameMoves::calculate_score(my, my_result);
    }

    println!("Opponent score: {opp_score}");
    println!("My score: {my_score}");

}
